import { describe, expect, test } from "vitest";
import { computeEntitlement, GRACE_MS } from "./entitlement.js";

const NOW = Date.parse("2026-10-10T12:00:00Z");
const DAY = 24 * 60 * 60 * 1000;

describe("computeEntitlement", () => {
  test("a user with no subscription is not premium", () => {
    expect(computeEntitlement(null, NOW)).toEqual({
      premium: false,
      reason: "none",
      accessEndsAt: null,
    });
  });

  test.each(["trialing", "active", "past_due"] as const)(
    "status %s is premium",
    (status) => {
      expect(computeEntitlement({ status }, NOW)).toEqual({
        premium: true,
        reason: "active",
        accessEndsAt: null,
      });
    },
  );

  test("paid-through: a canceled subscription keeps access until its period end", () => {
    // Cancelled mid-period, paid through to tomorrow — access runs to the
    // period end, and the grace window closes 7 days after that.
    const endedAt = NOW + DAY;
    expect(computeEntitlement({ status: "canceled", endedAt }, NOW)).toEqual({
      premium: true,
      reason: "paid_through",
      accessEndsAt: endedAt + GRACE_MS,
    });
  });

  test("grace: six days after the subscription ended the user is still premium", () => {
    const endedAt = NOW - 6 * DAY;
    expect(computeEntitlement({ status: "canceled", endedAt }, NOW)).toEqual({
      premium: true,
      reason: "grace",
      accessEndsAt: endedAt + GRACE_MS,
    });
  });

  test("the 7-day grace ends exactly at the boundary", () => {
    // One millisecond before the window closes, still premium…
    const endedAt = NOW - GRACE_MS + 1;
    expect(computeEntitlement({ status: "canceled", endedAt }, NOW).premium).toBe(
      true,
    );
    // …and at the boundary itself, not.
    const justOver = NOW - GRACE_MS;
    expect(computeEntitlement({ status: "canceled", endedAt: justOver }, NOW)).toEqual({
      premium: false,
      reason: "expired",
      accessEndsAt: justOver + GRACE_MS,
    });
  });

  test("eight days past the end the user is expired", () => {
    const endedAt = NOW - 8 * DAY;
    expect(computeEntitlement({ status: "unpaid", endedAt }, NOW)).toEqual({
      premium: false,
      reason: "expired",
      accessEndsAt: endedAt + GRACE_MS,
    });
  });

  test("a paused subscription inside its grace window is premium", () => {
    const endedAt = NOW - DAY;
    expect(computeEntitlement({ status: "paused", endedAt }, NOW).premium).toBe(true);
  });

  test("an entitled-looking status with no end anchor never claims a window", () => {
    expect(computeEntitlement({ status: "active" }, NOW).premium).toBe(true);
  });

  test("a non-entitled status with no end anchor grants nothing", () => {
    expect(computeEntitlement({ status: "canceled" }, NOW)).toEqual({
      premium: false,
      reason: "none",
      accessEndsAt: null,
    });
  });
});
