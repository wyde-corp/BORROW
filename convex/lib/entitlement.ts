/**
 * Premium access, computed from what the webhooks have written down.
 *
 * The rules are the product's, fixed in the design session:
 *
 * - `trialing`, `active`, and `past_due` are premium — `past_due` because
 *   Bachs is still retrying the charge and the user has not stopped paying
 *   on purpose.
 * - When the subscription leaves that set, a 7-day window opens from
 *   `endedAt` (the period end after a cancel-at-period-end, or the moment
 *   the status flipped otherwise). Re-subscribing inside it simply returns
 *   the user to `active`; nothing needs to be repaired.
 * - After the window, premium ends. An unknown future status grants
 *   nothing: billing defaults to closed, not open.
 *
 * Pure on purpose — no `ctx`, no clock of its own — so the boundary cases
 * are testable to the millisecond.
 */

export const GRACE_MS = 7 * 24 * 60 * 60 * 1000;

const ENTITLED = new Set(["trialing", "active", "past_due"]);

/** Statuses that are premium right now — the same set the rules read. */
export function isEntitledStatus(status: string | null | undefined): boolean {
  return status !== null && status !== undefined && ENTITLED.has(status);
}

export type EntitlementReason =
  /** No subscription row, or one with no status. */
  | "none"
  /** Inside a live subscription. */
  | "active"
  /** Subscription ended, but the paid-through period has not. */
  | "paid_through"
  /** Inside the 7-day grace window. */
  | "grace"
  /** The grace window has closed. */
  | "expired";

export type Entitlement = {
  premium: boolean;
  reason: EntitlementReason;
  /** When premium access ends, if a window is running (past for `expired`). */
  accessEndsAt: number | null;
};

/** The slice of a `subscriptions` row this function reads. */
export type SubscriptionState = {
  status?: string | null | undefined;
  endedAt?: number | null | undefined;
};

export function computeEntitlement(
  sub: SubscriptionState | null,
  now: number,
): Entitlement {
  if (!sub) {
    return { premium: false, reason: "none", accessEndsAt: null };
  }

  const status = sub.status ?? null;
  if (isEntitledStatus(status)) {
    return { premium: true, reason: "active", accessEndsAt: null };
  }

  const endedAt = sub.endedAt ?? null;
  if (endedAt === null) {
    // Not entitled, and nothing says when — claim nothing.
    return { premium: false, reason: "none", accessEndsAt: null };
  }

  const windowEnd = endedAt + GRACE_MS;
  if (now < endedAt) {
    return { premium: true, reason: "paid_through", accessEndsAt: windowEnd };
  }
  if (now < windowEnd) {
    return { premium: true, reason: "grace", accessEndsAt: windowEnd };
  }
  return { premium: false, reason: "expired", accessEndsAt: windowEnd };
}
