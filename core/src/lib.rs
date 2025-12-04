
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
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),

}