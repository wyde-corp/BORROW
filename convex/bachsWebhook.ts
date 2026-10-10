import { internal } from "./_generated/api";
import { httpAction, ActionCtx } from "./_generated/server";
import { verifyBachsSignature } from "./lib/signature";
import { json, requireEnv } from "./lib/http";

/**
 * The webhook endpoint — the only writer of billing state that matters.
 *
 * Order of operations is the security contract: read the *raw* body, verify
 * `X-Bachs-Signature-V2` against it, and only then parse. An unsigned or
 * mis-signed delivery is a 400 with no side effects; a signed one is applied
 * idempotently inside `applyWebhookEvent`, so Bachs's retries and replays
 * converge on the same rows.
 */
export const bachsWebhook = httpAction(async (ctx: ActionCtx, request: Request) => {
  let secret: string;
  try {
    secret = requireEnv("BACHS_WEBHOOK_SECRET");
  } catch {
    return json({ error: "webhook signing secret is not configured" }, 500);
  }

  const rawBody = await request.text();
  const valid = await verifyBachsSignature({
    header: request.headers.get("X-Bachs-Signature-V2"),
    rawBody,
    secret,
    nowSeconds: Math.floor(Date.now() / 1000),
  });
  if (!valid) return json({ error: "invalid signature" }, 400);

  let event: { id?: unknown; type?: unknown; data?: unknown };
  try {
    event = JSON.parse(rawBody);
  } catch {
    return json({ error: "body is not JSON" }, 400);
  }
  if (typeof event.id !== "string" || typeof event.type !== "string") {
    return json({ error: "unrecognized event envelope" }, 400);
  }

  const result = await ctx.runMutation(internal.subscriptions.applyWebhookEvent, {
    eventId: event.id,
    type: event.type,
    data: event.data ?? null,
  });

  return json({ received: true, ...result });
});
