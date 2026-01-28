use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::PullError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    DevTo,
}

impl Platform {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DevTo => "devto",
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Platform {
    type Err = PullError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "devto" | "dev.to" | "dev" => Ok(Self::DevTo),
            _ => Err(PullError::UnsupportedPlatform(s.to_string())),
        }
    }
}
