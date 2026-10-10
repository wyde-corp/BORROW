import { v } from "convex/values";
import { internalMutation, internalQuery, MutationCtx } from "./_generated/server";
import type { Doc } from "./_generated/dataModel";
import {
  computeEntitlement,
  isEntitledStatus,
  type Entitlement,
} from "./lib/entitlement";
import {
  parseInvoiceSubscriptionId,
  parseSubscriptionEvent,
} from "./lib/parseEvents";

type SubscriptionRow = Doc<"subscriptions">;

/**
 * Applies one Bachs webhook delivery.
 *
 * Deliveries are at-least-once and can arrive out of order, so:
 *
 * - the event id lands in `webhookEvents` first, in the same mutation —
 *   a duplicate returns `duplicate` and changes nothing;
 * - every fact from the payload is written by *locating* the row (by
 *   subscription id, then customer id, then the customer's email), never by
 *   assuming which event came first;
 * - a delivery that matches nothing we know is still recorded, but answers
 *   `ignored` — failing it would make Bachs retry forever.
 *
 * The return value is the ledger status, not the database's: `created` and
 * `updated` mean a row moved, `ignored` means the event was harmless.
 */
export const applyWebhookEvent = internalMutation({
  args: {
    eventId: v.string(),
    type: v.string(),
    data: v.any(),
  },
  handler: async (ctx, { eventId, type, data }) => {
    const seen = await ctx.db
      .query("webhookEvents")
      .withIndex("by_event", (q) => q.eq("eventId", eventId))
      .unique();
    if (seen) return { status: "duplicate" as const };

    await ctx.db.insert("webhookEvents", { eventId, receivedAt: Date.now() });

    switch (type) {
      case "customer.subscription.created":
      case "customer.subscription.updated":
        return await applySubscription(ctx, data, false);
      case "customer.subscription.deleted":
        return await applySubscription(ctx, data, true);
      case "invoice.payment_failed":
        return await applyPaymentFailed(ctx, data);
      default:
        // `invoice.paid` and everything else: recorded above, no state to move.
        return { status: "ignored" as const };
    }
  },
});

type ApplyCtx = MutationCtx;
type ApplyResult = { status: "created" | "updated" | "ignored" };

async function applySubscription(
  ctx: ApplyCtx,
  data: unknown,
  deleted: boolean,
): Promise<ApplyResult> {
  const parsed = parseSubscriptionEvent(data);
  if (!parsed) return { status: "ignored" };

  let status = parsed.status;
  if (deleted) {
    // A delete is an end no matter what the payload claims.
    status = status && !isEntitledStatus(status) ? status : "canceled";
  }

  const located = await locateSubscription(ctx, parsed);
  const now = Date.now();

  if (!located) {
    // Known shape, unknown human: nothing to write it to.
    return { status: "ignored" };
  }
  const { row, created } = located;

  const patch: Partial<SubscriptionRow> = { updatedAt: now };
  if (parsed.subscriptionId) patch.subscriptionId = parsed.subscriptionId;
  if (parsed.customerId) patch.bachsCustomerId = parsed.customerId;
  if (status) patch.status = status;
  if (parsed.cancelAtPeriodEnd !== null) {
    patch.cancelAtPeriodEnd = parsed.cancelAtPeriodEnd;
  }
  if (parsed.periodEnd !== null) patch.currentPeriodEnd = parsed.periodEnd;
  if (status && !isEntitledStatus(status)) {
    // The grace anchor: the paid-through period end when there is one,
    // otherwise the moment we learned the subscription stopped.
    patch.endedAt = parsed.periodEnd ?? now;
  }

  await ctx.db.patch(row._id, patch);
  return { status: created ? "created" : "updated" };
}

/** Finds the user's subscription row — or creates it, flagged as born here. */
async function locateSubscription(
  ctx: ApplyCtx,
  parsed: NonNullable<ReturnType<typeof parseSubscriptionEvent>>,
): Promise<{ row: SubscriptionRow; created: boolean } | null> {
  if (parsed.subscriptionId) {
    const bySubscription = await ctx.db
      .query("subscriptions")
      .withIndex("by_subscription", (q) => q.eq("subscriptionId", parsed.subscriptionId!))
      .first();
    if (bySubscription) return { row: bySubscription, created: false };
  }

  if (parsed.customerId) {
    const byCustomer = await ctx.db
      .query("subscriptions")
      .withIndex("by_customer", (q) => q.eq("bachsCustomerId", parsed.customerId!))
      .first();
    if (byCustomer) return { row: byCustomer, created: false };
  }

  if (!parsed.customerEmail) return null;
  const user = await ctx.db
    .query("users")
    .withIndex("by_email", (q) => q.eq("email", parsed.customerEmail!.toLowerCase()))
    .unique();
  if (!user) return null;

  const byUser = await ctx.db
    .query("subscriptions")
    .withIndex("by_user", (q) => q.eq("userId", user._id))
    .first();
  if (byUser) return { row: byUser, created: false };

  // The checkout's customer row may not have been saved yet — this delivery
  // is what creates it, carrying the customer id forward.
  const inserted = await ctx.db.insert("subscriptions", {
    userId: user._id,
    bachsCustomerId: parsed.customerId ?? undefined,
    updatedAt: Date.now(),
  });
  return { row: (await ctx.db.get(inserted))!, created: true };
}

async function applyPaymentFailed(ctx: ApplyCtx, data: unknown): Promise<ApplyResult> {
  const subscriptionId = parseInvoiceSubscriptionId(data);
  if (!subscriptionId) return { status: "ignored" };

  const row = await ctx.db
    .query("subscriptions")
    .withIndex("by_subscription", (q) => q.eq("subscriptionId", subscriptionId))
    .first();
  if (!row) return { status: "ignored" };

  // Only an entitled subscription can fall *into* arrears; one already
  // canceled has nothing to downgrade.
  if (!isEntitledStatus(row.status)) return { status: "ignored" };

  await ctx.db.patch(row._id, { status: "past_due", updatedAt: Date.now() });
  return { status: "updated" };
}

/** Saves the Bachs customer id created during checkout. */
export const saveCustomer = internalMutation({
  args: { userId: v.id("users"), customerId: v.string() },
  handler: async (ctx, { userId, customerId }) => {
    const existing = await ctx.db
      .query("subscriptions")
      .withIndex("by_user", (q) => q.eq("userId", userId))
      .first();
    if (existing) {
      await ctx.db.patch(existing._id, { bachsCustomerId: customerId, updatedAt: Date.now() });
    } else {
      await ctx.db.insert("subscriptions", {
        userId,
        bachsCustomerId: customerId,
        updatedAt: Date.now(),
      });
    }
  },
});

/**
 * Everything the app needs to know about one user's billing: the raw row
 * (for display) plus the computed entitlement (for access).
 */
export const state = internalQuery({
  args: { userId: v.id("users") },
  handler: async (ctx, { userId }) => {
    const row = await ctx.db
      .query("subscriptions")
      .withIndex("by_user", (q) => q.eq("userId", userId))
      .first();

    const entitlement: Entitlement = computeEntitlement(
      row ? { status: row.status, endedAt: row.endedAt } : null,
      Date.now(),
    );

    return {
      entitlement,
      subscription: row
        ? {
            status: row.status ?? null,
            currentPeriodEnd: row.currentPeriodEnd ?? null,
            cancelAtPeriodEnd: row.cancelAtPeriodEnd ?? false,
            bachsCustomerId: row.bachsCustomerId ?? null,
            subscriptionId: row.subscriptionId ?? null,
          }
        : null,
    };
  },
});

/** The raw row, for checkout's "does this user already have a customer?" */
export const customerOf = internalQuery({
  args: { userId: v.id("users") },
  handler: async (ctx, { userId }) => {
    const row = await ctx.db
      .query("subscriptions")
      .withIndex("by_user", (q) => q.eq("userId", userId))
      .first();
    return {
      customerId: row?.bachsCustomerId ?? null,
      // Surfaces "Manage billing" only for someone who has billed before.
      hasBilling: Boolean(row?.bachsCustomerId || row?.subscriptionId),
    };
  },
});
