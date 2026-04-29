use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use core::MarvelError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub scope: String,
    pub exp: u64,
    pub iat: u64,
}

pub fn create_access_token(client_id: &str, scope: &str, expires_in_secs: u64, secret: &[u8]) -> Result<String, MarvelError> {
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
        &EncodingKey::from_secret(secret),
    )
    .map_err(|e| MarvelError::ProtoError(format!("JWT-Erstellung fehlgeschlagen: {e}")))
}

pub fn verify_access_token(token: &str, secret: &[u8]) -> Result<Claims, MarvelError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| MarvelError::ProtoError(format!("Ungültiges Token: {e}")))
}
