
#[derive(thiserror::Error, Debug)]
pub enum MarvelError {
    #[error("Network error occurred: {0}")]
    NetworkError(String),
    #[error("Proto error occurred: {0}")]
    ProtoError(String),
    // ApiError(String),
    // NotFound(String),
    // Unauthorized(String),
    // Unknown(String),
}