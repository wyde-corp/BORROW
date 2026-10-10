import { internal } from "../_generated/api";
import type { ActionCtx } from "../_generated/server";
import type { Id } from "../_generated/dataModel";

/** JSON response, the shape every endpoint answers in. */
export function json(data: unknown, status = 200): Response {
  return new Response(JSON.stringify(data), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}

/**
 * The user behind `Authorization: Bearer <session token>`, or `null`.
 *
 * Resolving goes through `internal.sessions.resolveSession` — the http
 * action itself has no `ctx.db`, and the sliding expiry is a write, which a
 * query could not do anyway.
 */
export async function currentUser(
  ctx: ActionCtx,
  request: Request,
): Promise<Id<"users"> | null> {
  const header = request.headers.get("Authorization");
  if (!header || !header.startsWith("Bearer ")) return null;
  const token = header.slice("Bearer ".length).trim();
  if (!token) return null;
  const resolved = await ctx.runMutation(internal.sessions.resolveSession, { token });
  return resolved ? resolved.userId : null;
}

/** Reads a JSON body, or `null` when it is absent or malformed. */
export async function jsonBody(request: Request): Promise<Record<string, unknown> | null> {
  try {
    const body = await request.json();
    return typeof body === "object" && body !== null && !Array.isArray(body)
      ? (body as Record<string, unknown>)
      : null;
  } catch {
    return null;
  }
}

/** A required env var, or a 500 the operator can act on. */
export function requireEnv(name: string): string {
  const value = process.env[name];
  if (!value) throw new Error(`${name} is not set on this deployment`);
  return value;
}
