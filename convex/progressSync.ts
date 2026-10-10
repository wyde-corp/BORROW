import { httpAction, ActionCtx } from "./_generated/server";
import { currentUser, json, jsonBody } from "./lib/http";
import { internal } from "./_generated/api";

type ProgressRow = {
  kind: "lesson" | "exercise";
  itemId: number;
  completedAt: number;
  xp: number;
};

/** Everything synced for the signed-in account — the pull half of sync. */
export const getProgress = httpAction(async (ctx: ActionCtx, request: Request) => {
  const userId = await currentUser(ctx, request);
  if (!userId) return json({ error: "unauthorized" }, 401);

  const rows = await ctx.runQuery(internal.progress.pull, { userId });
  return json({ rows });
});

/**
 * The push half: the desktop sends its complete local completion set and
 * the server union-merges it (see `internal.progress.merge`). A full-set
 * push — rather than a delta — means a lost, reordered or duplicated
 * delivery cannot corrupt anything: it is idempotent by construction.
 */
export const postProgress = httpAction(async (ctx: ActionCtx, request: Request) => {
  const userId = await currentUser(ctx, request);
  if (!userId) return json({ error: "unauthorized" }, 401);

  const body = await jsonBody(request);
  if (!body || !Array.isArray(body.rows)) {
    return json({ error: "expected { rows: [...] }" }, 400);
  }

  const rows = body.rows as ProgressRow[];
  if (rows.length > 5000) return json({ error: "too many rows" }, 400);

  // Shape checking is the `merge` mutation's `args` validator — Convex
  // runs it before the handler, so a bad payload surfaces as a validation
  // error rather than half-applied rows.
  try {
    const result = await ctx.runMutation(internal.progress.merge, { userId, rows });
    return json(result);
  } catch (err) {
    if (err instanceof Error && err.message.includes("Validator failed")) {
      return json({ error: "rows are malformed" }, 400);
    }
    throw err;
  }
});
