
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use log::info;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use proto::{authentication};
use marvel_error::error::MarvelError;
#[tokio::main]
async fn main() -> Result<(), MarvelError> {
    tracing_subscriber::fmt::init();
    let r = authentication::build_payload()?;
    dbg!("{}",&r);
    let v = authentication::serialize_payload(&r);
    dbg!("{}",v);
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/authenticate", post(create_user))
        .route("/health", get(health_check));

    info!("Listening on http://0.0.0.0:3000");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.map_err(|_err| MarvelError::AxumError("Axum Server konnte nicht gestartet werden".to_string()))
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}
