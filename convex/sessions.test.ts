import { afterEach, beforeEach, describe, expect, test } from "vitest";
import { convexTest, type TestConvex } from "convex-test";
import { internal } from "./_generated/api";
import schema from "./schema";

type Test = TestConvex<typeof schema>;
const modules = import.meta.glob("./**/!(*.*.*)*.*s");

const SECRET_KEY = "SESSION_HASH_KEY";

function makeTest(): Test {
  return convexTest(schema, modules);
}

async function makeUser(t: Test) {
  return await t.run(async (ctx) => {
    return await ctx.db.insert("users", {
      email: "ada@example.com",
      createdAt: Date.now(),
    });
  });
}

describe("sessions", () => {
  beforeEach(() => {
    process.env.SESSION_HASH_KEY = SECRET_KEY;
  });

  afterEach(() => {
    delete process.env.SESSION_HASH_KEY;
  });

  test("a created session resolves back to its user", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    const { token } = await t.mutation(internal.sessions.createSession, { userId });

    expect(await t.mutation(internal.sessions.resolveSession, { token })).toEqual({
      userId,
      expiresAt: expect.any(Number),
    });
  });

  test("the stored row never contains the raw token", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    const { token } = await t.mutation(internal.sessions.createSession, { userId });

    const rows = await t.run((ctx) => ctx.db.query("sessions").collect());
    expect(rows).toHaveLength(1);
    expect(rows[0].tokenHash).not.toBe(token);
    expect(JSON.stringify(rows)).not.toContain(token);
  });

  test("an unknown token resolves to nothing", async () => {
    const t = makeTest();
    expect(
      await t.mutation(internal.sessions.resolveSession, { token: "not-a-real-token" }),
    ).toBeNull();
  });

  test("an expired session resolves to nothing and is removed", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    const { token } = await t.mutation(internal.sessions.createSession, { userId });

    await t.run(async (ctx) => {
      const row = (await ctx.db.query("sessions").collect())[0];
      await ctx.db.patch(row._id, { expiresAt: Date.now() - 1 });
    });

    expect(await t.mutation(internal.sessions.resolveSession, { token })).toBeNull();
    expect(await t.run((ctx) => ctx.db.query("sessions").collect())).toHaveLength(0);
  });

  test("a session inside the touch window slides forward to a full 30 days", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    const { token } = await t.mutation(internal.sessions.createSession, { userId });

    const soon = Date.now() + 10 * 24 * 60 * 60 * 1000; // 10 days left
    await t.run(async (ctx) => {
      const row = (await ctx.db.query("sessions").collect())[0];
      await ctx.db.patch(row._id, { expiresAt: soon });
    });

    const resolved = await t.mutation(internal.sessions.resolveSession, { token });
    expect(resolved).not.toBeNull();
    expect(resolved!.expiresAt).toBeGreaterThan(Date.now() + 29 * 24 * 60 * 60 * 1000);
  });

  test("a session with plenty of life left is not rewritten", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    const { token } = await t.mutation(internal.sessions.createSession, { userId });

    const later = Date.now() + 20 * 24 * 60 * 60 * 1000; // outside the touch window
    await t.run(async (ctx) => {
      const row = (await ctx.db.query("sessions").collect())[0];
      await ctx.db.patch(row._id, { expiresAt: later });
    });

    const resolved = await t.mutation(internal.sessions.resolveSession, { token });
    expect(resolved!.expiresAt).toBe(later);
  });

  test("revoking a token kills it", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    const { token } = await t.mutation(internal.sessions.createSession, { userId });

    await t.mutation(internal.sessions.revokeSession, { token });
    expect(await t.mutation(internal.sessions.resolveSession, { token })).toBeNull();
  });

  test("session functions fail closed when the hash key is missing", async () => {
    const t = makeTest();
    const userId = await makeUser(t);
    delete process.env.SESSION_HASH_KEY;

    await expect(
      t.mutation(internal.sessions.createSession, { userId }),
    ).rejects.toThrow(/SESSION_HASH_KEY/);
  });
});
