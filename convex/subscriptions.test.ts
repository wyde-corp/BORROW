import { describe, expect, test } from "vitest";
import { convexTest, type TestConvex } from "convex-test";
import { internal } from "./_generated/api";
import schema from "./schema";

type Test = TestConvex<typeof schema>;
const modules = import.meta.glob("./**/!(*.*.*)*.*s");

function makeTest(): Test {
  return convexTest(schema, modules);
}

const DAY = 24 * 60 * 60 * 1000;

async function makeUser(t: Test, email = "ada@example.com") {
  return await t.run((ctx) =>
    ctx.db.insert("users", { email, createdAt: Date.now() }),
  );
}

function iso(msFromNow: number): string {
  return new Date(Date.now() + msFromNow).toISOString();
}

function subscriptionEvent(overrides: Record<string, unknown> = {}) {
  return {
    subscription_id: "sub_1",
    customer: { customer_id: "cust_1", email: "ada@example.com", name: "Ada" },
    status: "active",
    current_period_start: iso(-DAY),
    current_period_end: iso(DAY),
    cancel_at_period_end: false,
    metadata: {},
    ...overrides,
  };
}

async function apply(
  t: Test,
  eventId: string,
  type: string,
  data: unknown,
) {
  return await t.mutation(internal.subscriptions.applyWebhookEvent, {
    eventId,
    type,
    data,
  });
}

describe("applyWebhookEvent", () => {
  test("the first subscription event creates the row and grants premium", async () => {
    const t = makeTest();
    await makeUser(t);

    const result = await apply(t, "evt_1", "customer.subscription.created", subscriptionEvent());
    expect(result).toEqual({ status: "created" });

    const state = await t.query(internal.subscriptions.state, {
      userId: (await t.run((ctx) => ctx.db.query("users").collect()))[0]._id,
    });
    expect(state.subscription).toMatchObject({
      status: "active",
      bachsCustomerId: "cust_1",
      subscriptionId: "sub_1",
    });
    expect(state.entitlement.premium).toBe(true);
    expect(state.entitlement.reason).toBe("active");
  });

  test("a duplicate delivery is applied exactly once", async () => {
    const t = makeTest();
    await makeUser(t);

    const first = await apply(t, "evt_1", "customer.subscription.created", subscriptionEvent());
    const second = await apply(t, "evt_1", "customer.subscription.created", subscriptionEvent());

    expect(first).toEqual({ status: "created" });
    expect(second).toEqual({ status: "duplicate" });
    expect(await t.run((ctx) => ctx.db.query("subscriptions").collect())).toHaveLength(1);
    expect(await t.run((ctx) => ctx.db.query("webhookEvents").collect())).toHaveLength(1);
  });

  test("a status update rewrites the same row instead of adding one", async () => {
    const t = makeTest();
    await makeUser(t);
    await apply(t, "evt_1", "customer.subscription.created", subscriptionEvent());
    await apply(
      t,
      "evt_2",
      "customer.subscription.updated",
      subscriptionEvent({ status: "past_due" }),
    );

    const rows = await t.run((ctx) => ctx.db.query("subscriptions").collect());
    expect(rows).toHaveLength(1);
    expect(rows[0].status).toBe("past_due");

    const userId = rows[0].userId;
    const state = await t.query(internal.subscriptions.state, { userId });
    // Still premium: Bachs is retrying the charge.
    expect(state.entitlement).toMatchObject({ premium: true, reason: "active" });
  });

  test("a cancel anchored at a period end one day ago is inside grace", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    await apply(
      t,
      "evt_1",
      "customer.subscription.updated",
      subscriptionEvent({ status: "canceled", current_period_end: iso(-DAY) }),
    );

    const state = await t.query(internal.subscriptions.state, { userId });
    expect(state.entitlement).toMatchObject({ premium: true, reason: "grace" });
    expect(state.entitlement.accessEndsAt).toBeGreaterThan(Date.now());
  });

  test("a cancel whose period ended eight days ago is expired", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    await apply(
      t,
      "evt_1",
      "customer.subscription.updated",
      subscriptionEvent({ status: "canceled", current_period_end: iso(-8 * DAY) }),
    );

    const state = await t.query(internal.subscriptions.state, { userId });
    expect(state.entitlement).toMatchObject({ premium: false, reason: "expired" });
  });

  test("a cancel still inside its paid-through period keeps access", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    await apply(
      t,
      "evt_1",
      "customer.subscription.updated",
      subscriptionEvent({ status: "canceled", current_period_end: iso(3 * DAY) }),
    );

    const state = await t.query(internal.subscriptions.state, { userId });
    expect(state.entitlement).toMatchObject({ premium: true, reason: "paid_through" });
  });

  test("subscription.deleted ends access with no period to wait for", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    await apply(t, "evt_1", "customer.subscription.created", subscriptionEvent());
    await apply(
      t,
      "evt_2",
      "customer.subscription.deleted",
      subscriptionEvent({ status: undefined, current_period_end: undefined }),
    );

    const state = await t.query(internal.subscriptions.state, { userId });
    expect(state.subscription!.status).toBe("canceled");
    // Grace starts now and runs 7 days.
    expect(state.entitlement).toMatchObject({ premium: true, reason: "grace" });
  });

  test("a repeat non-entitled delivery does not restart the grace window", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    // Cancelled with a period end six days ago: one day of grace left.
    await apply(
      t,
      "evt_1",
      "customer.subscription.updated",
      subscriptionEvent({ status: "canceled", current_period_end: iso(-6 * DAY) }),
    );
    // A later no-op delivery of the same non-entitled status — dunning
    // retries send these — must leave the anchor where it was. Re-anchoring
    // to "now" would hand out a fresh seven days on every redelivery.
    await apply(
      t,
      "evt_2",
      "customer.subscription.updated",
      subscriptionEvent({ status: "canceled", current_period_end: undefined }),
    );

    const state = await t.query(internal.subscriptions.state, { userId });
    expect(state.entitlement).toMatchObject({ premium: true, reason: "grace" });
    expect(state.entitlement.accessEndsAt!).toBeLessThan(Date.now() + 2 * DAY);
  });

  test("re-subscribing re-anchors the next grace window", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    await apply(
      t,
      "evt_1",
      "customer.subscription.updated",
      subscriptionEvent({ status: "canceled", current_period_end: iso(-6 * DAY) }),
    );
    // Renewal: back to active — the old anchor must not linger.
    await apply(t, "evt_2", "customer.subscription.updated", subscriptionEvent());
    expect(
      (await t.query(internal.subscriptions.state, { userId })).entitlement,
    ).toMatchObject({ premium: true, reason: "active" });

    // Second cancellation, this time with no period end in the payload:
    // the window opens from *this* end, not the first one.
    await apply(
      t,
      "evt_3",
      "customer.subscription.updated",
      subscriptionEvent({ status: "canceled", current_period_end: undefined }),
    );
    const state = await t.query(internal.subscriptions.state, { userId });
    expect(state.entitlement).toMatchObject({ premium: true, reason: "grace" });
    expect(state.entitlement.accessEndsAt!).toBeGreaterThan(Date.now() + 6 * DAY);
  });

  test("billing history means a subscription, not an abandoned checkout", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    // Checkout started: a customer id is saved before any payment.
    await t.mutation(internal.subscriptions.saveCustomer, {
      userId,
      customerId: "cust_1",
    });
    let billing = await t.query(internal.subscriptions.customerOf, { userId });
    expect(billing).toEqual({ customerId: "cust_1", hasBilling: false });

    // Payment landed: the webhook carried a subscription id.
    await apply(t, "evt_1", "customer.subscription.created", subscriptionEvent());
    billing = await t.query(internal.subscriptions.customerOf, { userId });
    expect(billing).toMatchObject({ customerId: "cust_1", hasBilling: true });
  });

  test("invoice.payment_failed moves an entitled subscription to past_due", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    await apply(t, "evt_1", "customer.subscription.created", subscriptionEvent());
    await apply(t, "evt_2", "invoice.paid", {
      invoice_id: "inv_1",
      subscription: { subscription_id: "sub_1" },
      status: "paid",
    });
    await apply(t, "evt_3", "invoice.payment_failed", {
      invoice_id: "inv_2",
      subscription: { subscription_id: "sub_1" },
      status: "open",
    });

    const rows = await t.run((ctx) => ctx.db.query("subscriptions").collect());
    expect(rows[0].status).toBe("past_due");
    const state = await t.query(internal.subscriptions.state, { userId });
    expect(state.entitlement.premium).toBe(true);
  });

  test("invoice.paid alone creates nothing", async () => {
    const t = makeTest();
    await makeUser(t);
    const result = await apply(t, "evt_1", "invoice.paid", {
      invoice_id: "inv_1",
      subscription: null,
      status: "paid",
    });

    expect(result).toEqual({ status: "ignored" });
    expect(await t.run((ctx) => ctx.db.query("subscriptions").collect())).toHaveLength(0);
    expect(await t.run((ctx) => ctx.db.query("webhookEvents").collect())).toHaveLength(1);
  });

  test("an event for nobody we know is recorded and ignored", async () => {
    const t = makeTest();
    const result = await apply(
      t,
      "evt_1",
      "customer.subscription.created",
      subscriptionEvent({ customer: { customer_id: "cust_x", email: "stranger@example.com" } }),
    );

    expect(result).toEqual({ status: "ignored" });
    expect(await t.run((ctx) => ctx.db.query("subscriptions").collect())).toHaveLength(0);
  });

  test("an unknown event type is recorded and ignored", async () => {
    const t = makeTest();
    await makeUser(t);
    const result = await apply(t, "evt_1", "payout.updated", { anything: true });

    expect(result).toEqual({ status: "ignored" });
    expect(await t.run((ctx) => ctx.db.query("webhookEvents").collect())).toHaveLength(1);
  });
});
