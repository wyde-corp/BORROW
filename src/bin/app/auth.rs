//! Sign-in: PKCE, the loopback redirect, and the session file.
//!
//! The flow is the standard native-app dance: the app binds a random
//! `127.0.0.1` port, sends the browser to the provider with a challenge
//! derived from a secret it keeps, and catches the redirect locally. The
//! code plus verifier then go to the backend, which holds the client secret
//! — nothing confidential ever ships in the binary.

use anyhow::{anyhow, Context as _, Result};
use base64::Engine as _;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};
use url::Url;

use crate::config;
use crate::services::remote::{Entitlement, ExchangeResponse, Me, SubscriptionInfo};

pub const REDIRECT_HOST: &str = "127.0.0.1";
pub const REDIRECT_PATH: &str = "/callback";

/// How long the browser gets to finish the dance before the listener gives
/// up — a closed tab should not leave a task waiting forever.
pub const LOGIN_TIMEOUT: Duration = Duration::from_secs(300);

const SUCCESS_HTML: &str = r#"<!doctype html>
<html><head><meta charset="utf-8"><title>BORROW_</title></head>
<body style="font-family: system-ui, sans-serif; display: grid; place-items: center; height: 100vh; margin: 0;">
<div style="text-align: center;">
<h1 style="font-weight: 700;">You're signed in</h1>
<p style="color: #666;">Close this window and return to BORROW_.</p>
</div>
</body></html>"#;

fn b64url(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// A PKCE pair: the secret stays here, the challenge travels in the URL.
pub struct Pkce {
    pub verifier: String,
    pub challenge: String,
}

pub fn pkce() -> Pkce {
    let mut secret = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);
    let verifier = b64url(&secret);
    let challenge = b64url(&Sha256::digest(verifier.as_bytes()));
    Pkce {
        verifier,
        challenge,
    }
}

/// The `state` param: proof the redirect belongs to *this* login attempt.
pub fn random_state() -> String {
    let mut secret = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut secret);
    b64url(&secret)
}

pub fn redirect_uri(port: u16) -> String {
    format!("http://{REDIRECT_HOST}:{port}{REDIRECT_PATH}")
}

/// The provider's authorize URL. Both providers accept loopback redirects
/// with a runtime-chosen port, so the port here is never pre-registered.
pub fn authorize_url(
    provider: &str,
    client_id: &str,
    redirect_uri: &str,
    state: &str,
    challenge: &str,
) -> Result<String> {
    let mut url = match provider {
        "google" => Url::parse("https://accounts.google.com/o/oauth2/v2/auth")?,
        "github" => Url::parse("https://github.com/login/oauth/authorize")?,
        other => return Err(anyhow!("unknown provider: {other}")),
    };

    {
        let mut query = url.query_pairs_mut();
        query
            .append_pair("client_id", client_id)
            .append_pair("redirect_uri", redirect_uri)
            .append_pair("state", state)
            .append_pair("code_challenge", challenge)
            .append_pair("code_challenge_method", "S256");
        match provider {
            "google" => {
                query
                    .append_pair("response_type", "code")
                    .append_pair("scope", "openid email profile");
            }
            "github" => {
                query.append_pair("scope", "read:user user:email");
            }
            _ => {}
        }
    }

    Ok(url.to_string())
}

/// A login attempt: URL to open, and the listener waiting for its answer.
pub struct PendingLogin {
    pub authorize_url: String,
    expected_state: String,
    code_verifier: String,
    redirect_uri: String,
    listener: tokio::net::TcpListener,
}

/// What the browser handed back, ready for the backend exchange.
pub struct Callback {
    pub code: String,
    pub code_verifier: String,
    pub redirect_uri: String,
}

/// Binds the loopback port and builds the authorize URL.
pub async fn begin(provider: &str, client_id: &str) -> Result<PendingLogin> {
    let listener = tokio::net::TcpListener::bind((REDIRECT_HOST, 0))
        .await
        .context("bind loopback listener")?;
    let port = listener.local_addr()?.port();
    let redirect_uri = redirect_uri(port);
    let pkce = pkce();
    let state = random_state();
    let authorize_url = authorize_url(provider, client_id, &redirect_uri, &state, &pkce.challenge)?;

    Ok(PendingLogin {
        authorize_url,
        expected_state: state,
        code_verifier: pkce.verifier,
        redirect_uri,
        listener,
    })
}

impl PendingLogin {
    /// Waits for the provider's redirect, bounded by `timeout`.
    pub async fn wait_for_code(self, timeout: Duration) -> Result<Callback> {
        tokio::time::timeout(timeout, self.accept())
            .await
            .map_err(|_| anyhow!("sign-in timed out — was the browser window closed?"))?
    }

    async fn accept(self) -> Result<Callback> {
        loop {
            let (stream, _) = self.listener.accept().await.context("accept")?;
            let mut reader = BufReader::new(stream);
            let mut request_line = String::new();
            if reader.read_line(&mut request_line).await.is_err() {
                continue;
            }

            // "GET /callback?code=…&state=… HTTP/1.1" — the target is the
            // second whitespace-separated field.
            let target = request_line
                .split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .to_owned();
            let path = target.split('?').next().unwrap_or_default().to_owned();
            let stream = reader.into_inner();

            if path != REDIRECT_PATH {
                respond(stream, 404, "Not found").await;
                continue;
            }

            let params = match Url::parse(&format!("http://{REDIRECT_HOST}{target}")) {
                Ok(url) => url
                    .query_pairs()
                    .map(|(k, v)| (k.into_owned(), v.into_owned()))
                    .collect::<HashMap<_, _>>(),
                Err(_) => continue,
            };

            if let Some(error) = params.get("error") {
                respond(
                    stream,
                    200,
                    "Sign-in was cancelled. You can close this window.",
                )
                .await;
                return Err(anyhow!("sign-in cancelled at the provider ({error})"));
            }

            let Some(code) = params.get("code").cloned() else {
                respond(stream, 400, "Missing code.").await;
                continue;
            };

            // State first: a redirect without our state is somebody else's
            // callback, and it must not reach the exchange.
            if params.get("state").map(String::as_str) != Some(self.expected_state.as_str()) {
                respond(stream, 400, "State mismatch.").await;
                return Err(anyhow!("state mismatch on the loopback redirect"));
            }

            respond(stream, 200, SUCCESS_HTML).await;
            return Ok(Callback {
                code,
                code_verifier: self.code_verifier.clone(),
                redirect_uri: self.redirect_uri.clone(),
            });
        }
    }
}

async fn respond(mut stream: tokio::net::TcpStream, status: u16, body: &str) {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "Bad Request",
    };
    let head = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes()).await;
    let _ = stream.write_all(body.as_bytes()).await;
    let _ = stream.shutdown().await;
}

/// The desktop's side of a signed-in account: token, profile, and the last
/// entitlement the server computed (the offline cache — see Q6: no offline
/// lockout, the last-known state keeps working until the next successful
/// refresh).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredSession {
    pub token: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: u64,
    pub email: String,
    #[serde(default)]
    pub name: Option<String>,
    /// The picture to show on Account — provider's, or the user's own.
    #[serde(default, rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub entitlement: Option<Entitlement>,
    #[serde(default, rename = "hasBilling")]
    pub has_billing: bool,
    #[serde(default)]
    pub subscription: Option<SubscriptionInfo>,
    #[serde(default, rename = "fetchedAt")]
    pub fetched_at: u64,
}

impl StoredSession {
    /// Builds the file contents from the login exchange. Entitlement is
    /// filled in by the first refresh — the exchange does not carry it.
    pub fn from_exchange(response: ExchangeResponse) -> Result<Self> {
        let user = response
            .user
            .ok_or_else(|| anyhow!("login response carried no profile"))?;
        Ok(Self {
            token: response.token,
            expires_at: response.expires_at,
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
            entitlement: None,
            has_billing: false,
            subscription: None,
            fetched_at: 0,
        })
    }

    /// Folds a fresh `GET /entitlement` answer into the cache.
    pub fn apply_me(&mut self, me: &Me) {
        self.email = me.user.email.clone();
        self.name = me.user.name.clone();
        self.avatar_url = me.user.avatar_url.clone();
        self.entitlement = Some(me.entitlement.clone());
        self.has_billing = me.has_billing;
        self.subscription = me.subscription.clone();
        self.fetched_at = now_ms();
    }

    /// The cached premium flag. Absent cache means free — a signed-in user
    /// whose refresh has never landed (offline first launch) starts locked
    /// and unlocks on the next successful fetch; entitlement is not
    /// recomputed locally, so grace never expires offline.
    pub fn premium(&self) -> bool {
        self.entitlement
            .as_ref()
            .map(|entitlement| entitlement.premium)
            .unwrap_or(false)
    }
}

/// `<data dir>/session.json` — beside the database, nowhere else.
pub fn session_path() -> PathBuf {
    config::app_data_dir().join("session.json")
}

pub fn load() -> Option<StoredSession> {
    load_from(&session_path())
}

/// Writes the session with owner-only permissions (0600) via a temp file +
/// rename, so a crash mid-write cannot leave a truncated file behind.
pub fn save(session: &StoredSession) -> Result<()> {
    save_to(&session_path(), session)
}

pub fn clear() {
    clear_at(&session_path());
}

pub fn load_from(path: &Path) -> Option<StoredSession> {
    let bytes = std::fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

pub fn save_to(path: &Path, session: &StoredSession) -> Result<()> {
    let directory = path
        .parent()
        .ok_or_else(|| anyhow!("session path has no parent"))?;
    std::fs::create_dir_all(directory).context("create data directory")?;

    let bytes = serde_json::to_vec_pretty(session).context("serialize session")?;
    let temp = path.with_extension("json.tmp");
    std::fs::write(&temp, &bytes).context("write session")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&temp, std::fs::Permissions::from_mode(0o600))
            .context("restrict session file")?;
    }

    std::fs::rename(&temp, path).context("replace session file")?;
    Ok(())
}

pub fn clear_at(path: &Path) {
    let _ = std::fs::remove_file(path);
}

/// Epoch milliseconds — the one clock both the session cache and the
/// paywall's countdown read.
pub(crate) fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{authorize_url, begin, clear_at, load_from, pkce, save_to, StoredSession};
    use crate::services::remote::Me;
    use std::collections::HashMap;
    use std::time::Duration;
    use tokio::io::AsyncWriteExt as _;
    use url::Url;

    #[test]
    fn pkce_verifier_and_challenge_are_base64url_shapes() {
        let pkce = pkce();
        // 32 bytes → 43 characters without padding; both halves of the pair
        // must be URL-safe, or the provider sees a mangled challenge.
        assert_eq!(pkce.verifier.len(), 43);
        assert_eq!(pkce.challenge.len(), 43);
        for value in [&pkce.verifier, &pkce.challenge] {
            assert!(value
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
        }
        assert_ne!(pkce.verifier, pkce.challenge);
    }

    #[test]
    fn google_authorize_url_carries_the_whole_dance() {
        let url = authorize_url(
            "google",
            "client-123",
            "http://127.0.0.1:9999/callback",
            "state-xyz",
            "challenge-abc",
        )
        .expect("google url builds");

        let parsed = Url::parse(&url).expect("url parses");
        assert_eq!(parsed.host_str(), Some("accounts.google.com"));
        let params: HashMap<String, String> = parsed
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        assert_eq!(params["client_id"], "client-123");
        assert_eq!(params["redirect_uri"], "http://127.0.0.1:9999/callback");
        assert_eq!(params["state"], "state-xyz");
        assert_eq!(params["code_challenge"], "challenge-abc");
        assert_eq!(params["code_challenge_method"], "S256");
        assert_eq!(params["response_type"], "code");
        assert!(params["scope"].contains("email"));
    }

    #[test]
    fn github_authorize_url_uses_the_email_scope() {
        let url = authorize_url(
            "github",
            "client-456",
            "http://127.0.0.1:9999/callback",
            "state-xyz",
            "challenge-abc",
        )
        .expect("github url builds");

        let parsed = Url::parse(&url).expect("url parses");
        assert_eq!(parsed.host_str(), Some("github.com"));
        let params: HashMap<String, String> = parsed
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        assert_eq!(params["client_id"], "client-456");
        assert_eq!(params["code_challenge_method"], "S256");
        assert!(params["scope"].contains("user:email"));
    }

    #[test]
    fn an_unknown_provider_refuses_instead_of_building_a_blank_url() {
        assert!(authorize_url("twitter", "id", "http://x", "s", "c").is_err());
    }

    /// The whole loopback seam: a client connects, sends the redirect the
    /// provider would, and the pending login hands back the code — while a
    /// redirect with the wrong state is refused.
    #[tokio::test]
    async fn the_loopback_listener_answers_with_the_code() {
        let pending = begin("google", "client-123").await.expect("binds");
        let authorize = Url::parse(&pending.authorize_url).expect("url");
        let redirect = authorize
            .query_pairs()
            .find(|(key, _)| key == "redirect_uri")
            .map(|(_, value)| value.into_owned())
            .expect("redirect_uri present");
        let state = pending.expected_state.clone();

        let waiter = tokio::spawn(pending.wait_for_code(Duration::from_secs(5)));

        let mut client = tokio::net::TcpStream::connect(
            redirect
                .trim_start_matches("http://")
                .split('/')
                .next()
                .expect("authority"),
        )
        .await
        .expect("connect");
        client
            .write_all(
                format!("GET /callback?code=the-code&state={state} HTTP/1.1\r\nHost: x\r\n\r\n")
                    .as_bytes(),
            )
            .await
            .expect("send");

        let callback = waiter.await.expect("join").expect("code arrives");
        assert_eq!(callback.code, "the-code");
        assert_eq!(callback.redirect_uri, redirect);
        assert!(!callback.code_verifier.is_empty());
    }

    #[tokio::test]
    async fn a_redirect_with_the_wrong_state_is_refused() {
        let pending = begin("github", "client-456").await.expect("binds");
        let authorize = Url::parse(&pending.authorize_url).expect("url");
        let redirect = authorize
            .query_pairs()
            .find(|(key, _)| key == "redirect_uri")
            .map(|(_, value)| value.into_owned())
            .expect("redirect_uri present");

        let waiter = tokio::spawn(pending.wait_for_code(Duration::from_secs(5)));

        let mut client = tokio::net::TcpStream::connect(
            redirect
                .trim_start_matches("http://")
                .split('/')
                .next()
                .expect("authority"),
        )
        .await
        .expect("connect");
        client
            .write_all(b"GET /callback?code=evil&state=forged HTTP/1.1\r\nHost: x\r\n\r\n")
            .await
            .expect("send");

        let result = waiter.await.expect("join");
        assert!(result.is_err(), "forged state must not yield a callback");
    }

    #[test]
    fn session_file_roundtrips_with_owner_only_permissions() {
        let path =
            std::env::temp_dir().join(format!("borrow-session-test-{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);

        let session = StoredSession {
            token: "secret-token".to_string(),
            expires_at: 123,
            email: "ada@example.com".to_string(),
            name: Some("Ada".to_string()),
            avatar_url: None,
            entitlement: None,
            has_billing: false,
            subscription: None,
            fetched_at: 0,
        };
        save_to(&path, &session).expect("saves");
        let loaded = load_from(&path).expect("loads");
        assert_eq!(loaded, session);

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = std::fs::metadata(&path)
                .expect("metadata")
                .permissions()
                .mode();
            assert_eq!(mode & 0o777, 0o600, "session file is owner-only");
        }

        clear_at(&path);
    }

    #[test]
    fn apply_me_folds_the_entitlement_into_the_cache() {
        let mut session = StoredSession {
            token: "t".into(),
            expires_at: 1,
            email: "old@example.com".into(),
            name: None,
            avatar_url: None,
            entitlement: None,
            has_billing: false,
            subscription: None,
            fetched_at: 0,
        };

        let me: Me = serde_json::from_str(
            r#"{
                "user": { "email": "ada@example.com", "name": "Ada", "avatarUrl": "https://example.com/pic.png" },
                "entitlement": { "premium": true, "reason": "active", "accessEndsAt": null },
                "subscription": { "status": "active", "currentPeriodEnd": 99, "cancelAtPeriodEnd": false },
                "hasBilling": true
            }"#,
        )
        .expect("fixture");

        session.apply_me(&me);
        assert!(session.premium());
        assert_eq!(session.email, "ada@example.com");
        assert_eq!(
            session.avatar_url.as_deref(),
            Some("https://example.com/pic.png"),
            "the face rides the refresh, not just the login"
        );
        assert!(session.has_billing);
        assert!(session.fetched_at > 0);
        assert_eq!(
            session.subscription.as_ref().unwrap().status.as_deref(),
            Some("active")
        );
    }

    #[test]
    fn without_a_cached_entitlement_the_user_is_free() {
        let session = StoredSession {
            token: "t".into(),
            expires_at: 1,
            email: "a@b.c".into(),
            name: None,
            avatar_url: None,
            entitlement: None,
            has_billing: false,
            subscription: None,
            fetched_at: 0,
        };
        assert!(!session.premium());
    }
}
