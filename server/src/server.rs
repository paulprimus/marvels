use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_extra::protobuf::Protobuf;
use log::info;
use proto::authentication::security::{
    AuthenticateRequest, AuthenticateResponse,
    AuthorizeRequest, AuthorizeResponse,
};
use uuid::Uuid;

use crate::authentication::verify_pkce;
use crate::token::{create_access_token, verify_access_token};
use crate::{AppState, AuthCodeEntry};

pub(crate) async fn authenticate(
    State(state): State<AppState>,
    Protobuf(payload): Protobuf<AuthenticateRequest>,
) -> (StatusCode, Protobuf<AuthenticateResponse>) {

    if !payload.code_challenge.is_empty() && payload.code_challenge_method != "S256" {
        return (
            StatusCode::BAD_REQUEST,
            Protobuf(AuthenticateResponse {
                subject: String::new(),
                error: "invalid_request".to_string(),
                error_description: "Nur S256 code_challenge_method wird unterstützt".to_string(),
            }),
        );
    }

    let auth_code = Uuid::new_v4().to_string();
    println!("auth_code: {}", auth_code);
    state.auth_codes.insert(auth_code.clone(), AuthCodeEntry {
        client_id: payload.client_id.clone(),
        code_challenge: payload.code_challenge.clone(),
        scope: String::new(),
    });

    info!("Auth-Code ausgestellt für client_id={}", payload.client_id);

    (
        StatusCode::OK,
        Protobuf(AuthenticateResponse {
            subject: auth_code,
            error: String::new(),
            error_description: String::new(),
        }),
    )
}

pub(crate) async fn authorize(
    State(state): State<AppState>,
    Protobuf(payload): Protobuf<AuthorizeRequest>,
) -> (StatusCode, Protobuf<AuthorizeResponse>) {

    info!("Authorizing payload={}", payload);

    if payload.grant_type == "authorization_code" {
        let entry = match state.auth_codes.remove(&payload.code) {
            Some((_, entry)) => entry,
            None => return (
                StatusCode::UNAUTHORIZED,
                Protobuf(AuthorizeResponse {
                    error: "invalid_grant".to_string(),
                    error_description: "Unbekannter oder abgelaufener Authorization Code".to_string(),
                    ..Default::default()
                }),
            ),
        };

        if !verify_pkce(&payload.code_verifier, &entry.code_challenge) {
            return (
                StatusCode::UNAUTHORIZED,
                Protobuf(AuthorizeResponse {
                    error: "invalid_grant".to_string(),
                    error_description: "PKCE-Verifikation fehlgeschlagen".to_string(),
                    ..Default::default()
                }),
            );
        }

        let scope = if payload.scope.is_empty() { "read".to_string() } else { payload.scope.clone() };
        let access_token = match create_access_token(&entry.client_id, &scope, state.token_expiry_secs, &state.jwt_secret) {
            Ok(t) => t,
            Err(e) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Protobuf(AuthorizeResponse {
                    error: "server_error".to_string(),
                    error_description: e.to_string(),
                    ..Default::default()
                }),
            ),
        };

        info!("Access Token ausgestellt für client_id={}", entry.client_id);

        return (
            StatusCode::OK,
            Protobuf(AuthorizeResponse {
                access_token,
                token_type: "Bearer".to_string(),
                expires_in: state.token_expiry_secs as i32,
                scope,
                ..Default::default()
            }),
        );
    }

    if payload.grant_type == "client_credentials" {
        let scope = if payload.scope.is_empty() { "read".to_string() } else { payload.scope.clone() };
        let access_token = match create_access_token(&payload.client_id, &scope, state.token_expiry_secs, &state.jwt_secret) {
            Ok(t) => t,
            Err(e) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Protobuf(AuthorizeResponse {
                    error: "server_error".to_string(),
                    error_description: e.to_string(),
                    ..Default::default()
                }),
            ),
        };

        return (
            StatusCode::OK,
            Protobuf(AuthorizeResponse {
                access_token,
                token_type: "Bearer".to_string(),
                expires_in: state.token_expiry_secs as i32,
                scope,
                ..Default::default()
            }),
        );
    }

    (
        StatusCode::BAD_REQUEST,
        Protobuf(AuthorizeResponse {
            error: "unsupported_grant_type".to_string(),
            error_description: format!("grant_type '{}' wird nicht unterstützt", payload.grant_type),
            ..Default::default()
        }),
    )
}

pub(crate) async fn protected(headers: HeaderMap, State(state): State<AppState>) -> Response {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => return (
            StatusCode::UNAUTHORIZED,
            "Authorization Header fehlt oder hat falsches Format (erwartet: Bearer <token>)",
        ).into_response(),
    };

    match verify_access_token(token, &state.jwt_secret) {
        Ok(claims) => {
            info!("Zugriff gewährt für subject={}, scope={}", claims.sub, claims.scope);
            (
                StatusCode::OK,
                format!("Willkommen, {}! Deine Berechtigungen: {}", claims.sub, claims.scope),
            ).into_response()
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            format!("Zugriff verweigert: {e}"),
        ).into_response(),
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    let auth = headers.get("Authorization")?.to_str().ok()?;
    auth.strip_prefix("Bearer ")
}
