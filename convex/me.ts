import { internal } from "./_generated/api";
import { httpAction, ActionCtx } from "./_generated/server";
import { currentUser, json } from "./lib/http";

/**
 * Who the desktop is talking to and what they may open: profile, computed
 * entitlement, and the raw subscription row for display. This is the
 * endpoint the app polls while checkout is open in the browser.
 */
export const me = httpAction(async (ctx: ActionCtx, request: Request) => {
  const userId = await currentUser(ctx, request);
  if (!userId) return json({ error: "unauthorized" }, 401);

  const user = await ctx.runQuery(internal.users.profile, { userId });
  if (!user) return json({ error: "unauthorized" }, 401);

  const subscription = await ctx.runQuery(internal.subscriptions.state, { userId });
  const billing = await ctx.runQuery(internal.subscriptions.customerOf, { userId });

  return json({
    user,
    entitlement: subscription.entitlement,
    subscription: subscription.subscription,
    hasBilling: billing.hasBilling,
  });
});
