import { v } from "convex/values";
import { internalMutation, internalQuery } from "./_generated/server";
import type { Id } from "./_generated/dataModel";

export type Provider = "google" | "github";

/**
 * Finds-or-creates the user behind one provider login.
 *
 * Matching is by verified email, not by provider id alone: the same human
 * signing in with Gmail first and GitHub later must land on one account with
 * two attached identities, not two accounts with half a profile each. Emails
 * are stored lowercase — both providers hand them over in whatever case the
 * user typed at signup.
 *
 * The provider id takes precedence: once `google:g-123` is attached to a
 * user, a later email change at the provider must not re-route that login to
 * somebody else's row.
 */
export const finishLogin = internalMutation({
  args: {
    provider: v.union(v.literal("google"), v.literal("github")),
    providerUserId: v.string(),
    email: v.string(),
    name: v.optional(v.string()),
    avatarUrl: v.optional(v.string()),
  },
  handler: async (ctx, args): Promise<Id<"users">> => {
    const email = args.email.trim().toLowerCase();
    if (!email) throw new Error("finishLogin requires a verified email");

    const existing = await ctx.db
      .query("oauthAccounts")
      .withIndex("by_provider", (q) =>
        q.eq("provider", args.provider).eq("providerUserId", args.providerUserId),
      )
      .unique();

    if (existing) {
      await ctx.db.patch(existing.userId, {
        ...(args.name !== undefined ? { name: args.name } : {}),
        ...(args.avatarUrl !== undefined ? { avatarUrl: args.avatarUrl } : {}),
      });
      return existing.userId;
    }

    const byEmail = await ctx.db
      .query("users")
      .withIndex("by_email", (q) => q.eq("email", email))
      .unique();

    const userId =
      byEmail?._id ??
      (await ctx.db.insert("users", {
        email,
        name: args.name,
        avatarUrl: args.avatarUrl,
        createdAt: Date.now(),
      }));

    await ctx.db.insert("oauthAccounts", {
      provider: args.provider,
      providerUserId: args.providerUserId,
      userId,
    });
    return userId;
  },
});

/** The profile the app shows after login — never the provider ids. */
export const profile = internalQuery({
  args: { userId: v.id("users") },
  handler: async (ctx, { userId }) => {
    const user = await ctx.db.get(userId);
    if (!user) return null;
    return {
      email: user.email,
      name: user.name ?? null,
      avatarUrl: user.avatarUrl ?? null,
    };
  },
});
