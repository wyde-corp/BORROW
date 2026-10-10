/**
 * Field extraction from Bachs webhook payloads — every field is optional in
 * practice (events differ, `subscription.deleted` carries less than
 * `updated`, and the REST objects use `id` where the webhooks use
 * `subscription_id`), so nothing here throws: unparsable data degrades to
 * `null` and the caller decides whether that is ignorable.
 */

type Dict = Record<string, unknown>;

function isDict(value: unknown): value is Dict {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function str(value: unknown): string | null {
  return typeof value === "string" && value !== "" ? value : null;
}

function bool(value: unknown): boolean | null {
  return typeof value === "boolean" ? value : null;
}

/** ISO 8601 → epoch milliseconds; anything unparseable is `null`. */
function isoMs(value: unknown): number | null {
  if (typeof value !== "string") return null;
  const parsed = Date.parse(value);
  return Number.isNaN(parsed) ? null : parsed;
}

export type ParsedSubscription = {
  subscriptionId: string | null;
  customerId: string | null;
  customerEmail: string | null;
  status: string | null;
  /** The billing period end, epoch ms. */
  periodEnd: number | null;
  cancelAtPeriodEnd: boolean | null;
};

/** Extracts the subscription facts from a `customer.subscription.*` payload. */
export function parseSubscriptionEvent(data: unknown): ParsedSubscription | null {
  if (!isDict(data)) return null;

  const customer = isDict(data.customer) ? data.customer : null;
  const parsed: ParsedSubscription = {
    subscriptionId: str(data.subscription_id) ?? str(data.id),
    customerId: customer ? str(customer.customer_id) : null,
    customerEmail: customer ? str(customer.email) : null,
    status: str(data.status),
    periodEnd: isoMs(data.current_period_end),
    cancelAtPeriodEnd: bool(data.cancel_at_period_end),
  };

  // Nothing to locate a user by means nothing to apply.
  if (!parsed.subscriptionId && !parsed.customerId && !parsed.customerEmail) {
    return null;
  }
  return parsed;
}

/** The subscription id inside an `invoice.*` payload (`data.subscription`). */
export function parseInvoiceSubscriptionId(data: unknown): string | null {
  if (!isDict(data)) return null;
  const subscription = data.subscription;
  if (!isDict(subscription)) return null;
  return str(subscription.subscription_id) ?? str(subscription.id);
}
