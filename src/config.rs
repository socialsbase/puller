use crate::error::{PullError, Result};
use std::env;

pub struct Config {
    pub devto_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            devto_api_key: env::var("DEVTO_API_KEY").ok(),
        }
    }

    pub fn devto_api_key(&self) -> Result<&str> {
        self.devto_api_key
            .as_deref()
            .ok_or_else(|| PullError::MissingConfig("DEVTO_API_KEY".to_string()))
    }
}
