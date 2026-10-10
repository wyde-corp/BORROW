import { internal } from "./_generated/api";
import { httpAction, ActionCtx } from "./_generated/server";
import { json, jsonBody, requireEnv } from "./lib/http";

/**
 * Exchanges an OAuth code (arriving over the desktop's loopback redirect)
 * for a BORROW session.
 *
 * The client secret never leaves this deployment: the desktop proves
 * possession of the code plus its PKCE verifier, and the swap with Google
 * or GitHub happens here. A login answers with `{ token, expiresAt, user }`
 * and nothing else — no provider access token is stored or forwarded.
 */
export const exchange = httpAction(async (ctx: ActionCtx, request: Request) => {
  const body = await jsonBody(request);
  if (!body) return json({ error: "expected a JSON body" }, 400);

  const { provider, code, codeVerifier, redirectUri } = body;
  if (typeof code !== "string" || typeof codeVerifier !== "string" || typeof redirectUri !== "string") {
    return json({ error: "code, codeVerifier and redirectUri are required" }, 400);
  }

  let identity: ProviderIdentity | null;
  try {
    identity =
      provider === "google"
        ? await googleIdentity(code, codeVerifier, redirectUri)
        : provider === "github"
          ? await githubIdentity(code, codeVerifier, redirectUri)
          : null;
  } catch (err) {
    return json({ error: err instanceof Error ? err.message : "token exchange failed" }, 400);
  }
  if (!identity) return json({ error: "unknown provider" }, 400);

  const userId = await ctx.runMutation(internal.users.finishLogin, {
    provider: identity.provider,
    providerUserId: identity.providerUserId,
    email: identity.email,
    name: identity.name,
    avatarUrl: identity.avatarUrl,
  });
  const session = await ctx.runMutation(internal.sessions.createSession, { userId });
  const user = await ctx.runQuery(internal.users.profile, { userId });

  return json({
    token: session.token,
    expiresAt: session.expiresAt,
    user,
  });
});

/** Signs out: the session row dies, so every copy of the token dies with it. */
export const logout = httpAction(async (ctx: ActionCtx, request: Request) => {
  const header = request.headers.get("Authorization");
  if (header && header.startsWith("Bearer ")) {
    const token = header.slice("Bearer ".length).trim();
    if (token) await ctx.runMutation(internal.sessions.revokeSession, { token });
  }
  return json({ ok: true });
});

type ProviderIdentity = {
  provider: "google" | "github";
  providerUserId: string;
  email: string;
  name?: string;
  avatarUrl?: string;
};

async function googleIdentity(
  code: string,
  codeVerifier: string,
  redirectUri: string,
): Promise<ProviderIdentity> {
  const clientId = requireEnv("GOOGLE_CLIENT_ID");
  const clientSecret = requireEnv("GOOGLE_CLIENT_SECRET");

  const tokens = await postForm("https://oauth2.googleapis.com/token", {
    code,
    client_id: clientId,
    client_secret: clientSecret,
    redirect_uri: redirectUri,
    grant_type: "authorization_code",
    code_verifier: codeVerifier,
  });
  const accessToken = requireString(tokens, "access_token");

  const profile = await getJson("https://openidconnect.googleapis.com/v1/userinfo", accessToken);
  const email = typeof profile.email === "string" ? profile.email : "";
  // Google distinguishes verified from unverified addresses; only a
  // verified one may key an account (finishLogin matches on email).
  if (!email || profile.email_verified !== true) {
    throw new Error("Google did not return a verified email");
  }
  return {
    provider: "google",
    providerUserId: String(profile.sub ?? ""),
    email,
    name: typeof profile.name === "string" ? profile.name : undefined,
    avatarUrl: typeof profile.picture === "string" ? profile.picture : undefined,
  };
}

async function githubIdentity(
  code: string,
  codeVerifier: string,
  redirectUri: string,
): Promise<ProviderIdentity> {
  const clientId = requireEnv("GITHUB_CLIENT_ID");
  const clientSecret = requireEnv("GITHUB_CLIENT_SECRET");

  const tokens = await postForm(
    "https://github.com/login/oauth/access_token",
    {
      client_id: clientId,
      client_secret: clientSecret,
      code,
      redirect_uri: redirectUri,
      code_verifier: codeVerifier,
    },
    { Accept: "application/json" },
  );
  const accessToken = requireString(tokens, "access_token");

  const user = await getJson("https://api.github.com/user", accessToken);
  const id = user.id;
  if (typeof id !== "number" && typeof id !== "string") throw new Error("GitHub returned no user id");

  // `/user.email` is often null; fall back to the verified list.
  let email = typeof user.email === "string" && user.email ? user.email : null;
  if (!email) {
    const emails = await getJson<Array<Record<string, unknown>>>(
      "https://api.github.com/user/emails",
      accessToken,
    );
    if (Array.isArray(emails)) {
      const verified = emails.filter((e) => e.verified === true && typeof e.email === "string");
      const primary = verified.find((e) => e.primary === true) ?? verified[0];
      email = primary ? (primary.email as string) : null;
    }
  }
  if (!email) throw new Error("GitHub did not return a verified email");

  return {
    provider: "github",
    providerUserId: String(id),
    email,
    name: typeof user.name === "string" && user.name ? user.name : undefined,
    avatarUrl:
      typeof user.avatar_url === "string" && user.avatar_url ? user.avatar_url : undefined,
  };
}

async function postForm(
  url: string,
  fields: Record<string, string>,
  extraHeaders: Record<string, string> = {},
): Promise<Record<string, unknown>> {
  const response = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
      Accept: "application/json",
      ...extraHeaders,
    },
    body: new URLSearchParams(fields).toString(),
  });
  const payload = (await response.json()) as Record<string, unknown>;
  if (!response.ok || typeof payload.access_token !== "string") {
    const message =
      typeof payload.error_description === "string"
        ? payload.error_description
        : typeof payload.error === "string"
          ? payload.error
          : `token endpoint returned ${response.status}`;
    throw new Error(message);
  }
  return payload;
}

async function getJson<T = Record<string, unknown>>(url: string, accessToken: string): Promise<T> {
  const response = await fetch(url, {
    headers: {
      Authorization: `Bearer ${accessToken}`,
      Accept: "application/json",
      "User-Agent": "BORROW",
    },
  });
  if (!response.ok) throw new Error(`profile endpoint returned ${response.status}`);
  return (await response.json()) as T;
}

function requireString(payload: Record<string, unknown>, key: string): string {
  const value = payload[key];
  if (typeof value !== "string" || !value) throw new Error(`token endpoint returned no ${key}`);
  return value;
}
