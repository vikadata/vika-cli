use thiserror::Error;

#[derive(Error, Debug)]
pub enum VIkaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error (code {code}): {message}")]
    Api { code: i64, message: String },
    #[error("Missing API token: set VIKA_TOKEN env var or pass token explicitly")]
    MissingToken,
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, VIkaError>;
