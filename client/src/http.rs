use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use proto::authentication::security;
use rand::Rng;
use sha2::{Digest, Sha256};

use core::MarvelError;

/// Ergebnis einer Authentifizierung – enthält Auth-Code und Code-Verifier
/// für den nächsten Schritt (authorize).
pub struct AuthResult {
    pub auth_code: String,
    pub code_verifier: String,
}

/// Client für die Kommunikation mit einem Marvels OAuth 2.1 Server.
///
/// # Beispiel
///
/// ```ignore
/// use marvels_client::MarvelsClient;
///
/// let client = MarvelsClient::new("http://localhost:3000");
///
/// // Schritt 1: Authentifizieren
/// let auth = client.authenticate("my-client-id", "my-secret").await?;
///
/// // Schritt 2: Autorisieren → Access Token
/// let token = client.authorize(&auth.auth_code, &auth.code_verifier, "my-client-id", "read").await?;
///
/// // Schritt 3: Geschützte Ressource abrufen
/// let response = client.call_protected(&token).await?;
/// ```
pub struct MarvelsClient {
    base_url: String,
    http: reqwest::Client,
}

impl MarvelsClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http: reqwest::Client::new(),
        }
    }

    /// Schritt 1: Client-Identität prüfen, Authorization Code erhalten.
    ///
    /// Generiert intern PKCE code_verifier und code_challenge.
    pub async fn authenticate(&self, client_id: &str, client_secret: &str) -> Result<AuthResult, MarvelError> {
        let code_verifier = generate_code_verifier();
        let code_challenge = generate_code_challenge(&code_verifier);

        let payload = security::AuthenticateRequest {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            code_challenge,
            code_challenge_method: "S256".to_string(),
            code: String::new(),
            redirect_uri: String::new(),
            code_verifier: String::new(),
        };

        let data = payload.encode_payload();

        let res = self.http
            .post(format!("{}/authenticate", self.base_url))
            .header("Content-Type", "application/protobuf")
            .body(data)
            .send()
            .await;

        match res {
            Ok(response) => {
                let bytes = response
                    .bytes()
                    .await
                    .map_err(|e| MarvelError::NetworkError(e.to_string()))?;
                let decoded = security::AuthenticateResponse::decode_payload(&bytes)
                    .map_err(|e| MarvelError::NetworkError(format!("Protobuf-Dekodierung fehlgeschlagen: {e}")))?;
                if !decoded.error.is_empty() {
                    return Err(MarvelError::NetworkError(format!("{}: {}", decoded.error, decoded.error_description)));
                }
                Ok(AuthResult {
                    auth_code: decoded.subject,
                    code_verifier,
                })
            }
            Err(e) => Err(MarvelError::NetworkError(e.to_string())),
        }
    }

    /// Schritt 2: Authorization Code + Code-Verifier gegen Access Token eintauschen.
    pub async fn authorize(&self, auth_code: &str, code_verifier: &str, client_id: &str, scope: &str) -> Result<String, MarvelError> {
        let payload = security::AuthorizeRequest {
            grant_type: "authorization_code".to_string(),
            client_id: client_id.to_string(),
            scope: scope.to_string(),
            refresh_token: String::new(),
            code: auth_code.to_string(),
            code_verifier: code_verifier.to_string(),
            redirect_uri: String::new(),
        };

        let data = payload.encode_payload();

        let res = self.http
            .post(format!("{}/authorize", self.base_url))
            .header("Content-Type", "application/protobuf")
            .body(data)
            .send()
            .await;

        match res {
            Ok(response) => {
                let bytes = response
                    .bytes()
                    .await
                    .map_err(|e| MarvelError::NetworkError(e.to_string()))?;
                let decoded = security::AuthorizeResponse::decode_payload(&bytes)
                    .map_err(|e| MarvelError::NetworkError(format!("Protobuf-Dekodierung fehlgeschlagen: {e}")))?;
                if !decoded.error.is_empty() {
                    return Err(MarvelError::NetworkError(format!("{}: {}", decoded.error, decoded.error_description)));
                }
                Ok(decoded.access_token)
            }
            Err(e) => Err(MarvelError::NetworkError(e.to_string())),
        }
    }

    /// Schritt 3: Geschützte Ressource mit Bearer Token aufrufen.
    pub async fn call_protected(&self, access_token: &str) -> Result<String, MarvelError> {
        let res = self.http
            .get(format!("{}/protected", self.base_url))
            .header("Authorization", format!("Bearer {access_token}"))
            .send()
            .await;

        match res {
            Ok(response) => response
                .text()
                .await
                .map_err(|e| MarvelError::NetworkError(e.to_string())),
            Err(e) => Err(MarvelError::NetworkError(e.to_string())),
        }
    }
}

fn generate_code_verifier() -> String {
    let mut bytes = [0u8; 64];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}
