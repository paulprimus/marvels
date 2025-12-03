use axum::{routing::{get, post}, Json, Router};
use axum::http::StatusCode;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use axum_extra::protobuf::Protobuf;
use proto::authentication::LoginPayload;

#[tokio::main]
pub async fn run_server() -> Result<(), MarvelError> {
    tracing_subscriber::fmt::init();


    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`

        // `POST /users` goes to `create_user`
        .route("/authenticate", post(authenticate));
        // .route("/health", get(health_check));

    info!("Listening on http://0.0.0.0:3000");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.map_err(|_err| MarvelError::AxumError("Axum Server konnte nicht gestartet werden".to_string()))
}

async fn authenticate(Protobuf(payload): Protobuf<LoginPayload>) -> StatusCode {
    // insert your application logic here
    dbg!(&payload);
    StatusCode::ACCEPTED
}

#[derive(thiserror::Error, Debug)]
pub enum MarvelError {
    #[error("Axum Error: {0}")]
    AxumError(String),
    #[error("Network error occurred: {0}")]
    NetworkError(String),
    #[error("Proto error occurred: {0}")]
    ProtoError(String),
    #[error("#[form] io::Error")]
    IOError(std::io::Error),
    // ApiError(String),
    // NotFound(String),
    // Unauthorized(String),
    // Unknown(String),
}