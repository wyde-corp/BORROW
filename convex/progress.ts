import { v } from "convex/values";
import { internalMutation, internalQuery } from "./_generated/server";

const rowValidator = v.object({
  kind: v.union(v.literal("lesson"), v.literal("exercise")),
  itemId: v.number(),
  completedAt: v.number(),
  xp: v.number(),
});

/**
 * One device's completions, as the desktop sends them: the whole local set,
 * every push.
 */
export const pull = internalQuery({
  args: { userId: v.id("users") },
  handler: async (ctx, { userId }) => {
    const rows = await ctx.db
      .query("progress")
      .withIndex("by_user", (q) => q.eq("userId", userId))
      .collect();
    return rows.map((row) => ({
      kind: row.kind,
      itemId: row.itemId,
      completedAt: row.completedAt,
      xp: row.xp,
    }));
  },
});

/**
 * Union merge: a completion is monotonic — it happened or it did not — so
 * two devices converge by adding what the other is missing, never by
 * racing over one row. Existing rows are left exactly as they are (an XP
 * value written by an older build is history, not a correction), and
 * duplicates inside one payload collapse to the first occurrence.
 */
export const merge = internalMutation({
  args: {
    userId: v.id("users"),
    rows: v.array(rowValidator),
  },
  handler: async (ctx, { userId, rows }) => {
    let inserted = 0;
    const seen = new Set<string>();

    for (const row of rows) {
      if (!Number.isFinite(row.itemId) || row.itemId <= 0) continue;
      const key = `${row.kind}:${row.itemId}`;
      if (seen.has(key)) continue;
      seen.add(key);

      const existing = await ctx.db
        .query("progress")
        .withIndex("by_user_item", (q) =>
          q.eq("userId", userId).eq("kind", row.kind).eq("itemId", row.itemId),
        )
        .first();
      if (existing) continue;

      // No module id on purpose: Convex does not host course content, so
      // the desktop resolves lesson → module from its own database on pull.
      await ctx.db.insert("progress", {
        userId,
        kind: row.kind,
        itemId: row.itemId,
        completedAt: row.completedAt,
        xp: row.xp,
      });
      inserted += 1;
    }

    return { inserted, seen: seen.size };
  },
});
