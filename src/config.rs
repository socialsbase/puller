use crate::error::{PullError, Result};
use std::env;

pub struct Config {
    pub forem_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            forem_api_key: env::var("VIBE_FOREM_API_KEY").ok(),
        }
    }

    pub fn forem_api_key(&self) -> Result<&str> {
        self.forem_api_key
            .as_deref()
            .ok_or_else(|| PullError::MissingConfig("VIBE_FOREM_API_KEY".to_string()))
    }
}
