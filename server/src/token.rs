use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use core::MarvelError;

/// Geheimer Schlüssel – in Produktion aus Umgebungsvariable laden!
const JWT_SECRET: &[u8] = b"marvel-secret-key-XAX";

/// JWT Claims (Payload)
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (client_id)
    pub sub: String,
    /// Granted scopes
    pub scope: String,
    /// Expiration (Unix-Timestamp)
    pub exp: u64,
    /// Issued at (Unix-Timestamp)
    pub iat: u64,
}

/// Erstellt ein signiertes JWT Access Token
pub fn create_access_token(client_id: &str, scope: &str, expires_in_secs: u64) -> Result<String, MarvelError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: client_id.to_string(),
        scope: scope.to_string(),
        exp: now + expires_in_secs,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|e| MarvelError::ProtoError(format!("JWT-Erstellung fehlgeschlagen: {e}")))
}

/// Verifiziert ein JWT Access Token und gibt die Claims zurück
pub fn verify_access_token(token: &str) -> Result<Claims, MarvelError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| MarvelError::ProtoError(format!("Ungültiges Token: {e}")))
}

