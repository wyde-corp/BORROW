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
deployment. Going live later is step 5b below — new values on a prod
deployment, nothing in the code.

Verify: `npx convex env ls` lists them all.

## 5b. Going live

The code defaults to sandbox (`BACHS_API_URL` falls back to
`sandbox-api.bachs.io`), so a misconfigured deployment can never charge a real
card. Live only happens when a **prod Convex deployment** carries live values.
Keep the dev deployment on sandbox — it stays your test-card playground.

1. **Create the prod deployment** (once):

   ```sh
   npx convex deploy            # promotes current code to the prod deployment
   npx convex env ls --prod     # confirms you're looking at prod
   ```

2. **Live Bachs setup** — mirror of step 4, live side:

   ```sh
   bachs login                  # live session; bachs whoami must say live
   ```

   - Recreate the product in live (5.00 USD, monthly — cadence is immutable;
     the sandbox product id does not exist in live). Copy the new `prod_…`.
   - Copy a **live API key** (`sk_live_…`) from the dashboard.
   - Register a **live webhook endpoint** pointing at the *prod* deployment:
     `https://<prod-deployment>.convex.site/webhooks/bachs`, same five events
     as step 4. Copy the new `whsec_…` — shown once.

3. **Set the prod env vars** — step 5 again, on prod, with live values:

   ```sh
   npx convex env set --prod SESSION_HASH_KEY        <fresh 64 hex chars>
   npx convex env set --prod GOOGLE_CLIENT_ID        <client id from step 2>
   npx convex env set --prod GOOGLE_CLIENT_SECRET    <client secret from step 2>
   npx convex env set --prod GITHUB_CLIENT_ID        <client id from step 3>
   npx convex env set --prod GITHUB_CLIENT_SECRET    <client secret from step 3>
   npx convex env set --prod BACHS_API_KEY           sk_live_…
   npx convex env set --prod BACHS_WEBHOOK_SECRET    whsec_…
   npx convex env set --prod BACHS_PRODUCT_ID        prod_…        # the live one
   npx convex env set --prod BACHS_API_URL           https://api.bachs.io
   npx convex env set --prod CHECKOUT_SUCCESS_URL    https://<prod-deployment>.convex.site/checkout/success
   npx convex env set --prod CHECKOUT_CANCEL_URL     https://<prod-deployment>.convex.site/checkout/cancel
   ```

   Never put `sk_live_…` or the live `BACHS_API_URL` on the dev deployment.

4. **Point the desktop at prod** — same three vars as step 6, prod values:

   ```sh
   BORROW_CONVEX_SITE_URL=https://<prod-deployment>.convex.site \
   BORROW_GOOGLE_CLIENT_ID=<google client id> \
   BORROW_GITHUB_CLIENT_ID=<github client id> \
   cargo run --bin borrow-app
   ```

5. **Verify with a real card** — sandbox test cards do not work in live.
   Run the step-7 checklist against prod: sign in, module 4 paywall, one real
   $5 charge, unlock within ~2 s, MANAGE BILLING opens the portal, and the
   Bachs dashboard shows the webhook delivery succeeding. A failed delivery
   shows there first.

Watch-outs once live:

- **Bachs disables webhook endpoints after ~72 h of continuous failure** —
  if the prod deployment sits idle/broken, fix deliveries before they
  accumulate or the endpoint is cut off.
- The 7-day grace window has only been proven by the unit suite and sandbox
  webhooks — the first real cancellation is worth watching end to end.
- A pending cancellation (cancel-at-period-end) is not yet shown in the app
  until the period actually ends.

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

Tagged releases get the same three from `release.yml`, which reads them as
repository variables (`BORROW_CONVEX_SITE_URL`, `BORROW_GOOGLE_CLIENT_ID`,
`BORROW_GITHUB_CLIENT_ID` — GitHub → Settings → Secrets and variables →
Actions → Variables) and refuses to build if any is missing.

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
