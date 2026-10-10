//! The desktop's client for the Convex backend.
//!
//! Every call is a plain HTTPS request against the deployment's HTTP-action
//! origin — the desktop never speaks Convex's own sync protocol and never
//! holds a deployment key. Authentication is the session token the login
//! exchange handed out, carried as `Authorization: Bearer …`.
//!
//! The module is compiled into the web binary too (it lives under
//! `services/` so both targets share it), hence the dead-code allowance at
//! the top: only the desktop calls it so far.

#![allow(dead_code)]

use anyhow::{anyhow, Context as _, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Premium access, as the backend computes it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entitlement {
    pub premium: bool,
    pub reason: String,
    /// When access ends (epoch ms) — present for `grace`/`paid_through`/
    /// `expired`. `expired` is in the past; the rest are in the future.
    #[serde(rename = "accessEndsAt")]
    pub access_ends_at: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct UserProfile {
    pub email: String,
    pub name: Option<String>,
}

/// The subscription row, for display only — access is `entitlement`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubscriptionInfo {
    pub status: Option<String>,
    #[serde(rename = "currentPeriodEnd")]
    pub current_period_end: Option<u64>,
    #[serde(rename = "cancelAtPeriodEnd")]
    pub cancel_at_period_end: Option<bool>,
}

/// `GET /entitlement` — the whole picture the app keeps on disk.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Me {
    pub user: UserProfile,
    pub entitlement: Entitlement,
    pub subscription: Option<SubscriptionInfo>,
    #[serde(rename = "hasBilling")]
    pub has_billing: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeResponse {
    pub token: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: u64,
    pub user: Option<UserProfile>,
}

/// One completion, as sync sends it. `kind` is `"lesson"` or `"exercise"`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProgressRow {
    pub kind: String,
    #[serde(rename = "itemId")]
    pub item_id: i64,
    /// Epoch milliseconds.
    #[serde(rename = "completedAt")]
    pub completed_at: u64,
    pub xp: i64,
}

/// Why a call failed in a way the UI must branch on: `401` means the session
/// is dead and the user must sign in again, everything else can ride the
/// cached state (no offline lockout — the network being down is not a
/// reason to close the app).
#[derive(Debug)]
pub enum FetchError {
    Unauthorized,
    Other(anyhow::Error),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::Unauthorized => write!(f, "session expired"),
            FetchError::Other(err) => write!(f, "{err}"),
        }
    }
}

/// POSTs an OAuth code to the backend, which swaps it with the provider
/// using its own client secret. Returns the session to store.
pub async fn exchange(
    site: &str,
    provider: &str,
    code: &str,
    code_verifier: &str,
    redirect_uri: &str,
) -> Result<ExchangeResponse> {
    let body = serde_json::json!({
        "provider": provider,
        "code": code,
        "codeVerifier": code_verifier,
        "redirectUri": redirect_uri,
    });
    post_json(&format!("{}/auth/exchange", trim(site)), None, body).await
}

/// The current profile + entitlement + billing flags for the session.
pub async fn me(site: &str, token: &str) -> Result<Me, FetchError> {
    match get_json::<Me>(&format!("{}/entitlement", trim(site)), Some(token)).await {
        Ok(me) => Ok(me),
        Err(err) => Err(classify(err)),
    }
}

/// Starts hosted checkout; returns the URL to open in the browser.
pub async fn checkout_url(site: &str, token: &str) -> Result<String> {
    let value: Value = post_json(
        &format!("{}/checkout", trim(site)),
        Some(token),
        json_null(),
    )
    .await?;
    url_field(value, "checkout_url")
}

/// Opens the customer portal (cancel / update card); returns its URL.
pub async fn portal_url(site: &str, token: &str) -> Result<String> {
    let value: Value =
        post_json(&format!("{}/portal", trim(site)), Some(token), json_null()).await?;
    url_field(value, "portal_url")
}

/// The account's completions, as stored server-side.
pub async fn pull_progress(site: &str, token: &str) -> Result<Vec<ProgressRow>> {
    let value: Value = get_json(&format!("{}/progress", trim(site)), Some(token)).await?;
    match value.get("rows") {
        Some(rows) => Ok(serde_json::from_value(rows.clone()).context("progress rows")?),
        None => Err(anyhow!("progress response has no rows")),
    }
}

/// Sends the full local completion set; the server union-merges it, so
/// replays and races are harmless.
pub async fn push_progress(site: &str, token: &str, rows: &[ProgressRow]) -> Result<()> {
    let body = serde_json::json!({ "rows": rows });
    let _: Value = post_json(&format!("{}/progress", trim(site)), Some(token), body).await?;
    Ok(())
}

/// Revokes the session server-side. Best-effort: clearing the local file
/// already signs this device out.
pub async fn logout(site: &str, token: &str) -> Result<()> {
    let _: Value = post_json(
        &format!("{}/auth/logout", trim(site)),
        Some(token),
        json_null(),
    )
    .await?;
    Ok(())
}

fn classify(err: anyhow::Error) -> FetchError {
    if err.downcast_ref::<Unauthorized>().is_some() {
        FetchError::Unauthorized
    } else {
        FetchError::Other(err)
    }
}

#[derive(Debug)]
struct Unauthorized;

impl std::fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unauthorized")
    }
}
impl std::error::Error for Unauthorized {}

fn json_null() -> Value {
    Value::Null
}

/// The backend answers `{ url: … }`; older naming would say `checkout_url`
/// — accept either so a field rename server-side cannot brick checkout.
fn url_field(value: Value, preferred: &str) -> Result<String> {
    value
        .get(preferred)
        .or_else(|| value.get("url"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| anyhow!("response has neither `{preferred}` nor `url`"))
}

fn trim(site: &str) -> &str {
    site.trim_end_matches('/')
}

async fn get_json<T: for<'de> Deserialize<'de>>(url: &str, token: Option<&str>) -> Result<T> {
    let client = reqwest::Client::new();
    let mut request = client.get(url);
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    parse(request.send().await.context("send")?).await
}

async fn post_json<T: for<'de> Deserialize<'de>>(
    url: &str,
    token: Option<&str>,
    body: Value,
) -> Result<T> {
    let client = reqwest::Client::new();
    let mut request = client.post(url).json(&body);
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    parse(request.send().await.context("send")?).await
}

async fn parse<T: for<'de> Deserialize<'de>>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    let text = response.text().await.context("read body")?;

    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(Unauthorized.into());
    }

    if !status.is_success() {
        let message = serde_json::from_str::<Value>(&text)
            .ok()
            .and_then(|value| {
                value
                    .get("error")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| format!("server returned {status}"));
        return Err(anyhow!(message));
    }

    serde_json::from_str::<T>(&text).with_context(|| format!("decode {status} response"))
}

#[cfg(test)]
mod tests {
    use super::Me;

    /// The `/entitlement` payload is the contract with the Convex backend;
    /// this pins the field renames (`accessEndsAt`, `hasBilling`, …) to a
    /// literal fixture instead of to whatever the struct happens to be.
    #[test]
    fn me_parses_the_real_payload_shape() {
        let fixture = r#"{
            "user": { "email": "ada@example.com", "name": "Ada" },
            "entitlement": { "premium": true, "reason": "grace", "accessEndsAt": 1791700000000 },
            "subscription": {
                "status": "canceled",
                "currentPeriodEnd": 1791000000000,
                "cancelAtPeriodEnd": true
            },
            "hasBilling": true
        }"#;

        let me: Me = serde_json::from_str(fixture).expect("fixture parses");
        assert!(me.entitlement.premium);
        assert_eq!(me.entitlement.reason, "grace");
        assert_eq!(me.entitlement.access_ends_at, Some(1791700000000));
        assert_eq!(me.user.email, "ada@example.com");
        assert_eq!(
            me.subscription.as_ref().unwrap().status.as_deref(),
            Some("canceled")
        );
        assert!(me.has_billing);
    }

    #[test]
    fn entitlement_without_a_window_parses_to_none() {
        let fixture = r#"{ "user": { "email": "a@b.c", "name": null },
                 "entitlement": { "premium": false, "reason": "none", "accessEndsAt": null },
                 "subscription": null, "hasBilling": false }"#;
        let me: Me = serde_json::from_str(fixture).expect("fixture parses");
        assert!(!me.entitlement.premium);
        assert_eq!(me.entitlement.access_ends_at, None);
        assert!(me.subscription.is_none());
    }
}
