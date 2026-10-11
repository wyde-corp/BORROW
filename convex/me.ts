import { internal } from "./_generated/api";
import { httpAction, ActionCtx } from "./_generated/server";
import { currentUser, json } from "./lib/http";

const MAX_AVATAR_BYTES = 3 * 1024 * 1024;

/**
 * Accepts a picture the user picked (raw bytes, one image), stores it in
 * Convex file storage, and points the account at the served URL. The
 * provider's picture stays untouched underneath — see `profile`.
 */
export const uploadAvatar = httpAction(async (ctx: ActionCtx, request: Request) => {
  const userId = await currentUser(ctx, request);
  if (!userId) return json({ error: "unauthorized" }, 401);

  const bytes = new Uint8Array(await request.arrayBuffer());
  if (bytes.byteLength === 0) return json({ error: "empty file" }, 400);
  if (bytes.byteLength > MAX_AVATAR_BYTES) {
    return json({ error: "image is larger than 3 MB" }, 400);
  }

  const type = request.headers.get("content-type") ?? "application/octet-stream";
  const blob = new Blob([bytes], { type });
  const stored = await ctx.storage.store(blob);
  const url = await ctx.storage.getUrl(stored);
  if (!url) return json({ error: "stored file could not be addressed" }, 500);
  await ctx.runMutation(internal.users.setAvatarCustom, {
    userId,
    avatarCustomUrl: url,
  });
  return json({ url });
});

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
