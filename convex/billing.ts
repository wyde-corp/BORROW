import { internal } from "./_generated/api";
import { httpAction, ActionCtx } from "./_generated/server";
import { currentUser, json, requireEnv } from "./lib/http";
import type { Id } from "./_generated/dataModel";

/**
 * The desktop's whole billing surface: two endpoints that mint hosted URLs.
 * Card numbers, prices and cancellation UI are Bachs's — here we only hold
 * the API key and point the browser at what it returns.
 *
 * `BACHS_API_URL` selects the environment (per deployment: the Convex dev
 * deployment carries sandbox credentials, prod carries live). It defaults to
 * the sandbox so a misconfigured deployment can never charge a real card.
 */
function bachsApi(): string {
  return process.env.BACHS_API_URL ?? "https://sandbox-api.bachs.io";
}

function bachsHeaders(): Record<string, string> {
  return {
    Authorization: `Bearer ${requireEnv("BACHS_API_KEY")}`,
    "Content-Type": "application/json",
    Accept: "application/json",
  };
}

async function bachsPost<T>(path: string, body?: unknown): Promise<T> {
  const response = await fetch(`${bachsApi()}${path}`, {
    method: "POST",
    headers: bachsHeaders(),
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  const payload = (await response.json().catch(() => null)) as T & { error?: unknown };
  if (!response.ok) {
    const message =
      payload && typeof payload === "object" && "error" in payload
        ? JSON.stringify(payload.error)
        : `Bachs returned ${response.status}`;
    throw new Error(message);
  }
  return payload;
}

/**
 * Makes sure the user has a Bachs customer, creating it on first checkout
 * (email + name only — the subscription itself never exists until the
 * hosted checkout completes).
 */
async function ensureCustomer(ctx: ActionCtx, userId: Id<"users">): Promise<string> {
  const customer = await ctx.runQuery(internal.subscriptions.customerOf, { userId });
  if (customer.customerId) return customer.customerId;

  const user = await ctx.runQuery(internal.users.profile, { userId });
  if (!user) throw new Error("no such user");

  const created = await bachsPost<{ customer_id: string }>("/v1/customers", {
    email: user.email,
    name: user.name ?? undefined,
    metadata: { user_id: userId },
  });
  await ctx.runMutation(internal.subscriptions.saveCustomer, {
    userId,
    customerId: created.customer_id,
  });
  return created.customer_id;
}

/** Opens hosted checkout for the $20/month product. */
export const checkout = httpAction(async (ctx: ActionCtx, request: Request) => {
  const userId = await currentUser(ctx, request);
  if (!userId) return json({ error: "unauthorized" }, 401);

  try {
    const customerId = await ensureCustomer(ctx, userId);
    const successUrl = requireEnv("CHECKOUT_SUCCESS_URL");

    const session = await bachsPost<{ checkout_url: string }>("/v1/checkout-sessions", {
      product_cart: [{ product_id: requireEnv("BACHS_PRODUCT_ID"), quantity: 1 }],
      customer: { customer_id: customerId },
      billing_currency: "USD",
      success_url: successUrl,
      cancel_url: process.env.CHECKOUT_CANCEL_URL || undefined,
      metadata: { user_id: userId },
    });
    if (!session.checkout_url) throw new Error("Bachs returned no checkout_url");
    return json({ url: session.checkout_url });
  } catch (err) {
    return json({ error: err instanceof Error ? err.message : "checkout failed" }, 502);
  }
});

/** Opens the Bachs customer portal — cancel and card updates live there. */
export const portal = httpAction(async (ctx: ActionCtx, request: Request) => {
  const userId = await currentUser(ctx, request);
  if (!userId) return json({ error: "unauthorized" }, 401);

  try {
    const customerId = await ensureCustomer(ctx, userId);
    const session = await bachsPost<{ url: string }>(
      `/v1/customers/${customerId}/portal-sessions`,
    );
    if (!session.url) throw new Error("Bachs returned no portal url");
    return json({ url: session.url });
  } catch (err) {
    return json({ error: err instanceof Error ? err.message : "portal failed" }, 502);
  }
});

/**
 * The page Bachs redirects to after hosted checkout. Static HTML on
 * purpose: the desktop notices the payment by polling entitlement, so this
 * page's only job is to tell the human to go back to the app.
 */
export const checkoutSuccess = httpAction(async () => page(
  "You're subscribed",
  "Your modules are unlocking now. Return to BORROW_ — this window can close.",
));

/** The cancel counterpart: no payment, no harm, same instruction. */
export const checkoutCancel = httpAction(async () => page(
  "Checkout cancelled",
  "No charge was made. Return to BORROW_ — this window can close.",
));

function page(title: string, body: string): Response {
  return new Response(
    `<!doctype html>
<html>
<head><meta charset="utf-8"><title>BORROW_</title></head>
<body style="font-family: system-ui, sans-serif; display: grid; place-items: center; height: 100vh; margin: 0; background: #0b0b0f; color: #fafafa;">
  <div style="text-align: center; max-width: 28rem; padding: 1rem;">
    <div style="font-weight: 700; color: #7c8cff; letter-spacing: .05em;">BORROW_</div>
    <h1 style="font-size: 1.5rem; margin: .75rem 0;">${title}</h1>
    <p style="color: #999;">${body}</p>
  </div>
</body>
</html>`,
    { status: 200, headers: { "Content-Type": "text/html; charset=utf-8" } },
  );
}
