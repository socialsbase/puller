use thiserror::Error;

#[derive(Error, Debug)]
pub enum PullError {
    #[error("API error: {0}")]
    Api(String),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML serialization error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing configuration: {0}")]
    MissingConfig(String),

    #[error("Invalid date format: {0}")]
    InvalidDate(String),

    #[error("Article not found: {0}")]
    NotFound(String),

    #[error("Rate limited, retry after {0} seconds")]
    RateLimited(u64),

    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),
}

pub type Result<T> = std::result::Result<T, PullError>;
