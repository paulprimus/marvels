use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use proto::authentication::security;
use rand::Rng;
use sha2::{Digest, Sha256};

use core::MarvelError;

pub async fn authenticate(client_id: &str, client_secret: &str) -> Result<String, MarvelError> {
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

    let payload = security::AuthenticateRequest {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        code_challenge,
        code_challenge_method: "S256".to_string(),
        code: String::new(),
        redirect_uri: String::new(),
        code_verifier: String::new(), // Erst beim /authorize-Request senden!
    };

    let data: Vec<u8> = payload.encode_payload();

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3000/authenticate")
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
            // Gibt "auth_code | code_verifier" zurück damit der Aufrufer beide Werte hat
            Ok(format!("{} | {}", decoded.subject, code_verifier))
        }
        Err(e) => Err(MarvelError::NetworkError(e.to_string())),
    }
}

/// Schritt 2: auth_code + code_verifier gegen Access Token eintauschen
pub async fn authorize(auth_code: &str, code_verifier: &str, client_id: &str, scope: &str) -> Result<String, MarvelError> {
    let payload = security::AuthorizeRequest {
        grant_type: "authorization_code".to_string(),
        client_id: client_id.to_string(),
        scope: scope.to_string(),
        refresh_token: String::new(),
        code: auth_code.to_string(),
        code_verifier: code_verifier.to_string(), // Jetzt den Originalwert senden
        redirect_uri: String::new(),
    };


    let data = payload.encode_payload();

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3000/authorize")
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

/// Schritt 3: Geschützte Ressource mit Bearer Token aufrufen
pub async fn call_protected(access_token: &str) -> Result<String, MarvelError> {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3000/protected")
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

/// Schritt 1: Zufälligen code_verifier generieren (43–128 Zeichen, URL-safe)
pub fn generate_code_verifier() -> String {
    let mut bytes = [0u8; 64]; // 64 bytes → 86 Base64url-Zeichen
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Schritt 2: code_challenge = BASE64URL(SHA256(code_verifier))
pub fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}
