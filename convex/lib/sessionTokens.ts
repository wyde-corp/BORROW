/**
 * Session token minting and hashing — pure helpers, Web Crypto only (the
 * Convex runtime has no `node:crypto`).
 *
 * The token the desktop holds is 32 random bytes, base64url. What the
 * database stores is HMAC-SHA256 of it under `SESSION_HASH_KEY`: a stolen
 * table cannot be replayed as logins, and the key can be rotated without
 * invalidating the formula.
 */

/** Fail closed: no key, no sessions — never an unhashed token at rest. */
function requiredKey(): string {
  const secret = process.env.SESSION_HASH_KEY;
  if (!secret) throw new Error("SESSION_HASH_KEY is not set");
  return secret;
}

export function newSessionToken(): string {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  return base64url(bytes);
}

export async function hashToken(token: string): Promise<string> {
  const key = await crypto.subtle.importKey(
    "raw",
    new TextEncoder().encode(requiredKey()),
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"],
  );
  const digest = await crypto.subtle.sign(
    "HMAC",
    key,
    new TextEncoder().encode(token),
  );
  return [...new Uint8Array(digest)].map((b) => b.toString(16).padStart(2, "0")).join("");
}

function base64url(bytes: Uint8Array): string {
  let binary = "";
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}
