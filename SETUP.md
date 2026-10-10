# BORROW_ — auth & payments setup

One-time steps to make sign-in and the $5/month subscription work, then the
day-to-day dev loop. Work top to bottom; each step says how to verify it.

## 0. Prerequisites

```sh
node --version          # 18+ (24 tested)
npm i -g @bachs/cli     # the bachs CLI
```

## 1. Convex deployment

```sh
npx convex dev          # first run: prompts for login, creates a project, wires .env.local
```

- **Unit tests** (`npm test`) need no deployment at all.
- **Everything below needs a Convex *cloud* dev deployment** (public
  `https://…convex.site` URL): OAuth redirects and Bachs webhooks must both
  reach the endpoint from outside your machine. A purely local backend
  (`127.0.0.1`) cannot be given to Bachs as a `success_url` or webhook target.

Verify: `npx convex whoami` shows the project; `.env.local` has `CONVEX_URL`
/ `CONVEX_SITE_URL` with `https://`.

Keep `npx convex dev` running in a terminal while developing — it pushes
function changes live.

## 2. Google OAuth client

1. <https://console.cloud.google.com> → **APIs & Services → Credentials →
   Create credentials → OAuth client ID**.
2. Application type: **Desktop app** (this type permits `http://127.0.0.1`
   loopback redirects with any port — nothing to register per port).
3. Copy the **Client ID** and **Client secret**.

## 3. GitHub OAuth app

1. <https://github.com/settings/developers> → **OAuth Apps → New OAuth App**.
2. Homepage URL: `http://127.0.0.1` (anything local).
3. **Authorization callback URL: `http://127.0.0.1/callback`** — no port;
   GitHub ignores the port for loopback callbacks.
4. Copy the **Client ID** and generate a **Client secret**.

## 4. Bachs (sandbox)

```sh
bachs login --sandbox
bachs whoami            # must say sandbox
```

1. Create the product — dashboard (<https://app.bachs.io> → Products) or
   CLI (`bachs products create --help` for the current flags):
   - Name: **BORROW Pro**
   - Price: **5.00 USD**, billing cycle: **monthly** (cadence is immutable
     after creation — get it right the first time)
   - Copy the `prod_…` id.
2. Copy a **sandbox API key** (`sk_sandbox_…`) from the dashboard's API keys
   page.
3. Register the webhook endpoint — dashboard → Webhooks, or
   `bachs endpoints create --help`:
   - URL: `https://<your-deployment>.convex.site/webhooks/bachs`
   - Events: `customer.subscription.created`,
     `customer.subscription.updated`, `customer.subscription.deleted`,
     `invoice.paid`, `invoice.payment_failed`
   - Copy the **signing secret** (`whsec_…`) — shown once.

## 5. Convex environment variables

```sh
# Session token hashing key — generate one per deployment:
#   openssl rand -hex 32
npx convex env set SESSION_HASH_KEY        <64 hex chars>
npx convex env set GOOGLE_CLIENT_ID        <client id from step 2>
npx convex env set GOOGLE_CLIENT_SECRET    <client secret from step 2>
npx convex env set GITHUB_CLIENT_ID        <client id from step 3>
npx convex env set GITHUB_CLIENT_SECRET    <client secret from step 3>
npx convex env set BACHS_API_KEY           sk_sandbox_…
npx convex env set BACHS_WEBHOOK_SECRET    whsec_…
npx convex env set BACHS_PRODUCT_ID        prod_…
npx convex env set BACHS_API_URL           https://sandbox-api.bachs.io
npx convex env set CHECKOUT_SUCCESS_URL    https://<deployment>.convex.site/checkout/success
npx convex env set CHECKOUT_CANCEL_URL     https://<deployment>.convex.site/checkout/cancel
```

Each Convex deployment carries its own values — repeat step 5 on every new
deployment. Going live later is: new values on a prod deployment
(`sk_live_…`, `BACHS_API_URL=https://api.bachs.io`, recreated product and
endpoint), nothing in the code.

Verify: `npx convex env ls` lists them all.

## 6. Run the desktop app against it

```sh
# Dev: runtime env wins, so no rebuild between deployments.
BORROW_CONVEX_SITE_URL=https://<deployment>.convex.site \
BORROW_GOOGLE_CLIENT_ID=<google client id> \
BORROW_GITHUB_CLIENT_ID=<github client id> \
cargo run --bin borrow-app
```

Release builds bake the same three at compile time instead:

```sh
BORROW_CONVEX_SITE_URL=… BORROW_GOOGLE_CLIENT_ID=… BORROW_GITHUB_CLIENT_ID=… \
cargo build --release
```

(Only the **client ids** are baked — they are public. Secrets live in
Convex; the shipped binary holds none.)

## 7. Manual end-to-end check

1. **Login** — first launch shows the gate; Google and GitHub both land on
   Home; ACCOUNT shows the address you signed in with.
2. **Gating** — with no subscription, module 4 opens the paywall (SUBSCRIBE
   — $5/MONTH); modules 1–3 work normally.
3. **Pay** — hosted checkout opens in the browser. Sandbox cards: any card
   succeeds except the declines (`4000000000000002` etc.). The paywall
   screen unlocks within ~2 seconds of payment (webhook → entitlement
   poll).
4. **Manage billing** — ACCOUNT → MANAGE BILLING opens the Bachs portal;
   cancel there and the app shows the 7-day grace window on the paywall.
5. **Sync** — complete a lesson here, then sign in on another machine:
   the completion arrives after the launch sync.

## Troubleshooting

| Symptom | Where to look |
| --- | --- |
| "Sign-in is not configured for this build" | The three env vars of step 6 are unset at run/compile time. |
| Every login fails with `SESSION_HASH_KEY is not set` | Step 5 — the key is per-deployment; a fresh deployment needs it set. |
| Login fails at the provider | OAuth redirect URI: Google Desktop client, GitHub `http://127.0.0.1/callback` (no port). |
| `state mismatch` on the loopback | Two logins racing — finish or restart the app. |
| Webhook returns `invalid signature` | `BACHS_WEBHOOK_SECRET` vs the endpoint's `whsec_`. |
| Payment completes but the app stays locked | `bachs events list` — did the event arrive? Is `npx convex dev` pushing? Endpoint URL right? |
| Checkout errors immediately | `npx convex env ls` — `BACHS_API_KEY`, `BACHS_PRODUCT_ID`, `CHECKOUT_SUCCESS_URL` on *this* deployment. |
