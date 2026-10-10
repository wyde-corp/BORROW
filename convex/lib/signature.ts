/**
 * Verification of `X-Bachs-Signature-V2`, against the raw body and before
 * any JSON parsing — a signature checked over parsed JSON is checking a
 * different byte string than the one that arrived.
 *
 * Header shape: `t={unix-seconds},v1={hex}` — `v1` may repeat while secrets
 * rotate, so any match accepts. The signed message is `"{t}.{rawBody}"`,
 * HMAC-SHA256 hex, keyed by the endpoint's signing secret (`whsec_…`).
 *
 * Web Crypto only: Convex's runtime has no `node:crypto`.
 */

/** Docs' recommended window: reject deliveries more than 300s off-clock. */
export const SIGNATURE_TOLERANCE_SECONDS = 300;

export function verifyBachsSignature(args: {
  header: string | null;
  rawBody: string;
  secret: string;
  nowSeconds: number;
}): Promise<boolean> {
  const { header, rawBody, secret, nowSeconds } = args;
  if (!header) return Promise.resolve(false);

  let timestamp: number | null = null;
  const signatures: string[] = [];
  for (const part of header.split(",")) {
    const eq = part.indexOf("=");
    if (eq === -1) continue;
    const key = part.slice(0, eq).trim();
    const value = part.slice(eq + 1).trim();
    if (key === "t") {
      // Number("") is 0 and Number("12abc") is NaN — only a fully numeric
      // timestamp counts, and NaN must not slip through the tolerance check.
      const parsed = Number(value);
      if (value === "" || !Number.isFinite(parsed)) return Promise.resolve(false);
      timestamp = parsed;
    } else if (key === "v1") {
      signatures.push(value);
    }
  }

  if (timestamp === null || signatures.length === 0) return Promise.resolve(false);
  if (Math.abs(nowSeconds - timestamp) > SIGNATURE_TOLERANCE_SECONDS) {
    return Promise.resolve(false);
  }

  return hmacHex(secret, `${timestamp}.${rawBody}`).then((expected) =>
    signatures.some((candidate) => timingSafeEqualHex(candidate, expected)),
  );
}

async function hmacHex(secret: string, message: string): Promise<string> {
  const key = await crypto.subtle.importKey(
    "raw",
    new TextEncoder().encode(secret),
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"],
  );
  const digest = await crypto.subtle.sign("HMAC", key, new TextEncoder().encode(message));
  return [...new Uint8Array(digest)].map((b) => b.toString(16).padStart(2, "0")).join("");
}

/** Equal-length comparison that does not short-circuit on the first byte. */
function timingSafeEqualHex(a: string, b: string): boolean {
  if (a.length !== b.length) return false;
  let diff = 0;
  for (let i = 0; i < a.length; i++) diff |= a.charCodeAt(i) ^ b.charCodeAt(i);
  return diff === 0;
}
