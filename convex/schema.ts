import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  // One row per human. `email` is the verified address both providers attest
  // to, so Google and GitHub accounts for the same person meet here.
  users: defineTable({
    email: v.string(),
    name: v.optional(v.string()),
    avatarUrl: v.optional(v.string()),
    createdAt: v.number(),
  }).index("by_email", ["email"]),

  // Provider identities attached to a user. A user may hold one row per
  // provider; `finishLogin` finds-or-links by (provider, providerUserId).
  oauthAccounts: defineTable({
    provider: v.union(v.literal("google"), v.literal("github")),
    providerUserId: v.string(),
    userId: v.id("users"),
  }).index("by_provider", ["provider", "providerUserId"]),

  // Sessions store only the HMAC of the token the desktop holds: a leaked
  // database cannot be replayed as a login.
  sessions: defineTable({
    tokenHash: v.string(),
    userId: v.id("users"),
    expiresAt: v.number(),
    createdAt: v.number(),
  }).index("by_token", ["tokenHash"]),

  // One billing row per user. Written only by webhook deliveries (and the
  // customer id at first checkout) — never by the client.
  subscriptions: defineTable({
    userId: v.id("users"),
    bachsCustomerId: v.optional(v.string()),
    subscriptionId: v.optional(v.string()),
    // Free-form on purpose: Bachs has shipped statuses beyond the documented
    // set (e.g. `paused`), and a webhook the schema rejects is a webhook
    // that retries until the endpoint is disabled. `computeEntitlement`
    // grants nothing to a status it does not recognise.
    status: v.optional(v.string()),
    // When paid access ends (period end, or the moment the status left the
    // entitled set) — the anchor the 7-day grace counts from.
    endedAt: v.optional(v.number()),
    // The billing period Bachs reports, kept for display and to derive
    // `endedAt` on a cancel-at-period-end.
    currentPeriodEnd: v.optional(v.number()),
    cancelAtPeriodEnd: v.optional(v.boolean()),
    updatedAt: v.number(),
  }).index("by_user", ["userId"])
    .index("by_customer", ["bachsCustomerId"])
    .index("by_subscription", ["subscriptionId"]),

  // Idempotency ledger: webhook deliveries are at-least-once, so each event
  // id is applied once.
  webhookEvents: defineTable({
    eventId: v.string(),
    receivedAt: v.number(),
  }).index("by_event", ["eventId"]),

  // Completions synced from the desktop. Union-merged on push; no rows are
  // ever deleted, so two devices converge instead of racing.
  progress: defineTable({
    userId: v.id("users"),
    kind: v.union(v.literal("lesson"), v.literal("exercise")),
    itemId: v.number(),
    completedAt: v.number(),
    xp: v.number(),
  })
    .index("by_user", ["userId"])
    .index("by_user_item", ["userId", "kind", "itemId"]),
});
