
#[derive(thiserror::Error, Debug)]
pub enum MarvelError {
    #[error("Axum Error: {0}")]
    AxumError(String),
    #[error("Network error occurred: {0}")]
    NetworkError(String),
    #[error("Proto error occurred: {0}")]
    ProtoError(String),
    #[error("IO error occurred: {0}")]
    IOError(String),
}