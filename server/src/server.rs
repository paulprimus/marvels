use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{routing::{get, post}, Router};
use axum_extra::protobuf::Protobuf;
use core::MarvelError;
use dashmap::DashMap;
use log::info;
use proto::authentication::security::{
    AuthenticateRequest, AuthenticateResponse,
    AuthorizeRequest, AuthorizeResponse,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::authentication::verify_pkce;
use crate::token::{create_access_token, verify_access_token};

// Shared State

/// Wird per Arc zwischen allen Request-Handlern geteilt
#[derive(Clone)]
pub struct AppState {
    /// Speichert: auth_code → (client_id, code_challenge, scope)
    pub auth_codes: Arc<DashMap<String, AuthCodeEntry>>,
}

pub struct AuthCodeEntry {
    pub client_id: String,
    pub code_challenge: String,
    pub scope: String,
}

impl AppState {
    fn new() -> Self {
        AppState {
            auth_codes: Arc::new(DashMap::new()),
        }
    }
}

// Endpunkte

/// POST /authenticate
/// Schritt 1: Client-Identität prüfen, auth_code + code_challenge speichern
async fn authenticate(
    State(state): State<AppState>,
    Protobuf(payload): Protobuf<AuthenticateRequest>,
) -> (StatusCode, Protobuf<AuthenticateResponse>) {

    // Nur S256 erlaubt (OAuth 2.1 Pflicht)
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

    // TODO: client_id / client_secret gegen Datenbank prüfen

    // Auth-Code generieren und zusammen mit code_challenge speichern
    let auth_code = Uuid::new_v4().to_string();
    println!("auth_code: {}", auth_code);
    state.auth_codes.insert(auth_code.clone(), AuthCodeEntry {
        client_id: payload.client_id.clone(),
        code_challenge: payload.code_challenge.clone(),
        scope: String::new(), // Scope kommt beim AuthorizeRequest
    });

    info!("Auth-Code ausgestellt für client_id={}", payload.client_id);

    (
        StatusCode::OK,
        Protobuf(AuthenticateResponse {
            // subject enthält den auth_code – Client nutzt ihn im nächsten Schritt
            subject: auth_code,
            error: String::new(),
            error_description: String::new(),
        }),
    )
}

/// POST /authorize
/// Schritt 2: auth_code + code_verifier einlösen → Access Token ausstellen
async fn authorize(
    State(state): State<AppState>,
    Protobuf(payload): Protobuf<AuthorizeRequest>,
) -> (StatusCode, Protobuf<AuthorizeResponse>) {

    info!("Authorizing payload={}", payload);

    if payload.grant_type == "authorization_code" {
        // Auth-Code aus Cache holen (einmalig – danach löschen!)
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

        // PKCE-Verifikation: code_verifier muss zur gespeicherten code_challenge passen
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

        // Access Token als JWT ausstellen
        let scope = if payload.scope.is_empty() { "read".to_string() } else { payload.scope.clone() };
        let access_token = match create_access_token(&entry.client_id, &scope, 3600) {
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
                expires_in: 3600,
                scope,
                ..Default::default()
            }),
        );
    }

    // grant_type=client_credentials (kein PKCE nötig)
    if payload.grant_type == "client_credentials" {
        let scope = if payload.scope.is_empty() { "read".to_string() } else { payload.scope.clone() };
        let access_token = match create_access_token(&payload.client_id, &scope, 3600) {
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
                expires_in: 3600,
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

/// GET /protected
/// Schritt 3: Geschützte Ressource – nur mit gültigem Bearer Token zugänglich
async fn protected(headers: HeaderMap) -> Response {
    // Authorization: Bearer <token> auslesen
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => return (
            StatusCode::UNAUTHORIZED,
            "Authorization Header fehlt oder hat falsches Format (erwartet: Bearer <token>)",
        ).into_response(),
    };

    // JWT verifizieren
    match verify_access_token(token) {
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

/// Extrahiert den Bearer Token aus dem Authorization-Header
fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    let auth = headers.get("Authorization")?.to_str().ok()?;
    auth.strip_prefix("Bearer ")
}

// Server starten

pub async fn run_server() -> Result<(), MarvelError> {
    tracing_subscriber::fmt::init();

    let state = AppState::new();

    let app = Router::new()
        .route("/authenticate", post(authenticate))
        .route("/authorize", post(authorize))
        .route("/protected", get(protected))
        .with_state(state);

    info!("Listening on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .map_err(|_err| MarvelError::AxumError("Axum Server konnte nicht gestartet werden".to_string()))
}
