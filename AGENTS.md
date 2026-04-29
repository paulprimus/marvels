# AGENTS.md – Marvels

## Project

Rust workspace (edition 2024) implementing an OAuth 2.1 auth server with PKCE. Protobuf-over-HTTP communication between client and server.

## Crates

| Crate              | Role                                      |
|--------------------|-------------------------------------------|
| `core`             | Shared `MarvelError` enum                 |
| `proto`            | Prost codegen from `security.proto`       |
| `server` (`marvels_auth`) | Library: Axum Router builder for auth endpoints |
| `client` (`marvels_client`) | Library: HTTP client for Marvels server |

## Build prerequisites

- **`protoc` must be installed** — `proto/build.rs` compiles `security.proto` via prost-build. Without it the workspace won't compile.

## Commands

```bash
cargo build --workspace     # build all crates (codegen runs automatically)
cargo test -p server         # runs 3 PKCE tests in server/src/authentication.rs
```

Both `server` and `client` are **libraries only** — no `cargo run`. Embed them into your own applications.

## Library usage (`marvels_auth`)

The server crate exports `AuthRouterBuilder` — a builder that produces an Axum Router with `/authenticate`, `/authorize`, and `/protected` endpoints.

```rust
use marvels_auth::AuthRouterBuilder;
use rustls;

// jsonwebtoken v10 needs explicit crypto provider — install it BEFORE building the router
rustls::crypto::ring::default_provider()
    .install_default()
    .expect("CryptoProvider failed");

let auth_router = AuthRouterBuilder::new()
    .jwt_secret(b"your-secret-key")
    .token_expiry(3600) // optional, default: 1h
    .build();

// Mount into your app
let app = Router::new()
    .nest("/auth", auth_router)
    // ... your other routes
```

**Exported types**: `AuthRouterBuilder`, `AppState`, `AuthCodeEntry`, `Claims`, `verify_pkce`

## Library usage (`marvels_client`)

The client crate exports `MarvelsClient` — a typed HTTP client for the OAuth 2.1 flow.

```rust
use marvels_client::MarvelsClient;

let client = MarvelsClient::new("http://localhost:3000");

// Schritt 1: Authentifizieren → AuthResult { auth_code, code_verifier }
let auth = client.authenticate("my-client-id", "my-secret").await?;

// Schritt 2: Autorisieren → JWT Access Token
let token = client.authorize(&auth.auth_code, &auth.code_verifier, "my-client-id", "read").await?;

// Schritt 3: Geschützte Ressource abrufen
let response = client.call_protected(&token).await?;
```

**Exported types**: `MarvelsClient`, `AuthResult`

## Architecture notes

- **Endpoints**: `POST /authenticate`, `POST /authorize`, `GET /protected`
- **Auth codes**: stored in-memory (`DashMap`) — single-node only, no persistence. Consumed on use.
- **JWT tokens**: HS256, configurable expiry (default 1h). Secret must be provided via builder.
- **`jsonwebtoken v10` requires explicit crypto provider** — caller must install `ring` provider before JWT operations, otherwise they panic.
- **Communication is protobuf**, not JSON. Client sends `Content-Type: application/protobuf`.

## Testing

- Only unit tests: 3 PKCE verification tests in `server/src/authentication.rs`.
- No integration tests, no CI configuration.
- Run with `cargo test -p server`.

## Conventions

- Edition 2024 across all crates.
- No rustfmt.toml or clippy.toml — uses Rust defaults.
- Comments and error messages are in German.
- No README, no CI, no task runner.
