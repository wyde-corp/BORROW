import { describe, expect, test } from "vitest";
import { createHmac } from "node:crypto";
import { verifyBachsSignature } from "./signature.js";

const SECRET = "whsec_test_secret";
const BODY = JSON.stringify({ id: "evt_1", type: "customer.subscription.updated" });
const NOW = Math.floor(Date.parse("2026-10-10T12:00:00Z") / 1000);

function sign(secret: string, body: string, timestamp: number): string {
  // Fixture built with node:crypto, independently of the Web Crypto path
  // under test: the header is a known-good literal the implementation must
  // accept, not a value it produced itself.
  const v1 = createHmac("sha256", secret)
    .update(`${timestamp}.${body}`)
    .digest("hex");
  return `t=${timestamp},v1=${v1}`;
}

async function verify(header: string | null, body: string, secret = SECRET) {
  return verifyBachsSignature({ header, rawBody: body, secret, nowSeconds: NOW });
}

describe("verifyBachsSignature", () => {
  test("accepts a correctly signed body", async () => {
    expect(await verify(sign(SECRET, BODY, NOW), BODY)).toBe(true);
  });

  test("rejects a body that was altered after signing", async () => {
    const header = sign(SECRET, BODY, NOW);
    expect(await verify(header, BODY.replace("evt_1", "evt_2"))).toBe(false);
  });

  test("rejects a signature made with another secret", async () => {
    expect(await verify(sign("whsec_other", BODY, NOW), BODY)).toBe(false);
  });

  test("rejects a timestamp outside the 300-second tolerance", async () => {
    expect(await verify(sign(SECRET, BODY, NOW - 301), BODY)).toBe(false);
    expect(await verify(sign(SECRET, BODY, NOW - 300), BODY)).toBe(true);
  });

  test("rejects a timestamp too far in the future", async () => {
    expect(await verify(sign(SECRET, BODY, NOW + 301), BODY)).toBe(false);
  });

  test("accepts when any v1 in a rotating header matches", async () => {
    // Rotation overlap: Bachs may ship the outgoing and incoming secrets at
    // once; acceptance needs one match, not all.
    const good = sign(SECRET, BODY, NOW);
    const header = `${good},v1=${"0".repeat(64)}`;
    expect(await verify(header, BODY)).toBe(true);
  });

  test("rejects a missing or malformed header", async () => {
    expect(await verify(null, BODY)).toBe(false);
    expect(await verify("", BODY)).toBe(false);
    expect(await verify("t=not-a-number,v1=abc", BODY)).toBe(false);
    expect(await verify("v1=abc", BODY)).toBe(false);
  });
});
