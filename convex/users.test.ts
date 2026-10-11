import { describe, expect, test } from "vitest";
import { convexTest, type TestConvex } from "convex-test";
import { internal } from "./_generated/api";
import schema from "./schema";

type Test = TestConvex<typeof schema>;
const modules = import.meta.glob("./**/!(*.*.*)*.*s");

function makeTest(): Test {
  return convexTest(schema, modules);
}

const google = { provider: "google" as const, providerUserId: "g-123" };
const github = { provider: "github" as const, providerUserId: "gh-456" };

async function users(t: Test) {
  return await t.run((ctx) => ctx.db.query("users").collect());
}

async function accounts(t: Test) {
  return await t.run((ctx) => ctx.db.query("oauthAccounts").collect());
}

describe("finishLogin", () => {
  test("first login creates the user", async () => {
    const t = makeTest();
    const userId = await t.mutation(internal.users.finishLogin, {
      ...google,
      email: "ada@example.com",
      name: "Ada",
    });

    expect(await users(t)).toEqual([
      expect.objectContaining({ email: "ada@example.com", name: "Ada" }),
    ]);
    expect(await accounts(t)).toEqual([
      expect.objectContaining({ provider: "google", providerUserId: "g-123", userId }),
    ]);
  });

  test("the same provider account maps to the same user every time", async () => {
    const t = makeTest();
    const first = await t.mutation(internal.users.finishLogin, {
      ...google,
      email: "ada@example.com",
      name: "Ada",
    });
    const second = await t.mutation(internal.users.finishLogin, {
      ...google,
      email: "ada@example.com",
      name: "Ada L.",
    });

    expect(second).toBe(first);
    expect(await users(t)).toHaveLength(1);
    expect(await accounts(t)).toHaveLength(1);
    // The refreshed profile wins.
    expect((await users(t))[0].name).toBe("Ada L.");
  });

  test("a second provider with the same verified email attaches to one human", async () => {
    const t = makeTest();
    const first = await t.mutation(internal.users.finishLogin, {
      ...google,
      email: "ada@example.com",
    });
    const second = await t.mutation(internal.users.finishLogin, {
      ...github,
      email: "ada@example.com",
      name: "ada-gh",
    });

    expect(second).toBe(first);
    expect(await users(t)).toHaveLength(1);
    const linked = await accounts(t);
    expect(linked).toHaveLength(2);
    expect(linked.map((a) => a.provider).sort()).toEqual(["github", "google"]);
  });

  test("a different person gets their own user row", async () => {
    const t = makeTest();
    await t.mutation(internal.users.finishLogin, {
      ...google,
      email: "ada@example.com",
    });
    await t.mutation(internal.users.finishLogin, {
      provider: "google",
      providerUserId: "g-999",
      email: "grace@example.com",
    });

    expect(await users(t)).toHaveLength(2);
    expect(await accounts(t)).toHaveLength(2);
  });

  test("a custom picture outranks the provider's, and survives re-login", async () => {
    const t = makeTest();
    const google = { provider: "google" as const, providerUserId: "g-777" };
    const userId = await t.mutation(internal.users.finishLogin, {
      ...google,
      email: "ada@example.com",
      avatarUrl: "https://example.com/provider.png",
    });
    expect((await t.query(internal.users.profile, { userId }))!.avatarUrl).toBe(
      "https://example.com/provider.png",
    );

    await t.mutation(internal.users.setAvatarCustom, {
      userId,
      avatarCustomUrl: "https://convex.site/api/storage/chosen",
    });
    expect((await t.query(internal.users.profile, { userId }))!.avatarUrl).toBe(
      "https://convex.site/api/storage/chosen",
    );

    // A later login re-saves the provider's picture underneath — the
    // override must still win.
    await t.mutation(internal.users.finishLogin, {
      ...google,
      email: "ada@example.com",
      avatarUrl: "https://example.com/provider-again.png",
    });
    expect((await t.query(internal.users.profile, { userId }))!.avatarUrl).toBe(
      "https://convex.site/api/storage/chosen",
    );
  });

  test("emails are matched case-insensitively", async () => {
    const t = makeTest();
    const first = await t.mutation(internal.users.finishLogin, {
      ...google,
      email: "Ada@Example.com",
    });
    const second = await t.mutation(internal.users.finishLogin, {
      ...github,
      email: "ada@example.com",
    });

    expect(second).toBe(first);
    expect(await users(t)).toHaveLength(1);
    expect((await users(t))[0].email).toBe("ada@example.com");
  });
});
