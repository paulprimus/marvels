use axum::Router;
use axum::routing::{get, post};
use dashmap::DashMap;
use std::sync::Arc;

/// Eintrag für einen Authorization Code im Cache
pub struct AuthCodeEntry {
    pub client_id: String,
    pub code_challenge: String,
    pub scope: String,
}

/// Shared State zwischen allen Handlern
#[derive(Clone)]
pub struct AppState {
    pub auth_codes: Arc<DashMap<String, AuthCodeEntry>>,
    pub jwt_secret: Vec<u8>,
    pub token_expiry_secs: u64,
}

pub mod authentication;
mod token;
mod server;

/// Builder für den Auth-Router
///
/// # Beispiel
///
/// ```ignore
/// use marvels_auth::AuthRouterBuilder;
///
/// let router = AuthRouterBuilder::new()
///     .jwt_secret(b"mein-geheimes-secret")
///     .token_expiry(7200)
///     .build();
/// ```
pub struct AuthRouterBuilder {
    jwt_secret: Option<Vec<u8>>,
    token_expiry_secs: u64,
}

impl AuthRouterBuilder {
    pub fn new() -> Self {
        Self {
            jwt_secret: None,
            token_expiry_secs: 3600,
        }
    }

    pub fn jwt_secret(mut self, secret: impl AsRef<[u8]>) -> Self {
        self.jwt_secret = Some(secret.as_ref().to_vec());
        self
    }

    pub fn token_expiry(mut self, secs: u64) -> Self {
        self.token_expiry_secs = secs;
        self
    }

    pub fn build(self) -> Router {
        let jwt_secret = self.jwt_secret.expect("jwt_secret muss konfiguriert werden");

        let state = AppState {
            auth_codes: Arc::new(DashMap::new()),
            jwt_secret,
            token_expiry_secs: self.token_expiry_secs,
        };

        Router::new()
            .route("/authenticate", post(server::authenticate))
            .route("/authorize", post(server::authorize))
            .route("/protected", get(server::protected))
            .with_state(state)
    }
}

impl Default for AuthRouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
