import { v } from "convex/values";
import { internalMutation } from "./_generated/server";
import { hashToken, newSessionToken } from "./lib/sessionTokens";

/** A session lives 30 days from its last touch. */
export const SESSION_TTL_MS = 30 * 24 * 60 * 60 * 1000;
/**
 * Sliding expiry only rewrites a row once less than half the TTL remains —
 * an active user's session never quietly dies, and a hit per login does not
 * become a write per request.
 */
const TOUCH_WINDOW_MS = SESSION_TTL_MS / 2;

/** Creates a session for `userId`; the raw token leaves here and never returns. */
export const createSession = internalMutation({
  args: { userId: v.id("users") },
  handler: async (ctx, { userId }) => {
    const token = newSessionToken();
    const expiresAt = Date.now() + SESSION_TTL_MS;
    await ctx.db.insert("sessions", {
      tokenHash: await hashToken(token),
      userId,
      expiresAt,
      createdAt: Date.now(),
    });
    return { token, expiresAt };
  },
});

/**
 * Validates a token: unknown and expired both resolve to `null` (expired
 * rows are swept on the way out), a session inside the touch window slides
 * forward to a full TTL.
 */
export const resolveSession = internalMutation({
  args: { token: v.string() },
  handler: async (ctx, { token }) => {
    const tokenHash = await hashToken(token);
    const row = await ctx.db
      .query("sessions")
      .withIndex("by_token", (q) => q.eq("tokenHash", tokenHash))
      .unique();

    if (!row) return null;

    const now = Date.now();
    if (row.expiresAt <= now) {
      await ctx.db.delete(row._id);
      return null;
    }

    if (row.expiresAt < now + TOUCH_WINDOW_MS) {
      const expiresAt = now + SESSION_TTL_MS;
      await ctx.db.patch(row._id, { expiresAt });
      return { userId: row.userId, expiresAt };
    }

    return { userId: row.userId, expiresAt: row.expiresAt };
  },
});

/** Signs out: the row goes, so every copy of the token dies with it. */
export const revokeSession = internalMutation({
  args: { token: v.string() },
  handler: async (ctx, { token }) => {
    const tokenHash = await hashToken(token);
    const row = await ctx.db
      .query("sessions")
      .withIndex("by_token", (q) => q.eq("tokenHash", tokenHash))
      .unique();
    if (row) await ctx.db.delete(row._id);
  },
});
