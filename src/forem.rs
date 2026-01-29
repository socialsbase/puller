//! Forem instance definitions for all known Forem communities.
//!
//! This module provides the `ForemInstance` enum which represents all known
//! Forem-based communities, including dev.to and various forem.com sub-communities.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::PullError;

/// Known Forem instances and communities.
///
/// This enum supports all 17+ known forem.com communities, as well as custom
/// self-hosted Forem instances.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ForemInstance {
    /// dev.to - the original and largest Forem community
    DevTo,
    /// vibe.forem.com
    Vibe,
    /// open.forem.com
    Open,
    /// future.forem.com
    Future,
    /// gg.forem.com
    Gg,
    /// music.forem.com
    Music,
    /// popcorn.forem.com
    Popcorn,
    /// design.forem.com
    Design,
    /// zeroday.forem.com
    Zeroday,
    /// golf.forem.com
    Golf,
    /// crypto.forem.com
    Crypto,
    /// parenting.forem.com
    Parenting,
    /// core.forem.com
    Core,
    /// maker.forem.com
    Maker,
    /// hmpljs.forem.com
    Hmpljs,
    /// dumb.dev.to
    DumbDev,
    /// Custom Forem instance with a specified domain
    Custom { domain: String },
}

impl ForemInstance {
    /// Returns the API base URL for this Forem instance.
    #[must_use]
    pub fn base_url(&self) -> String {
        match self {
            Self::DevTo => "https://dev.to/api".to_string(),
            Self::Vibe => "https://vibe.forem.com/api".to_string(),
            Self::Open => "https://open.forem.com/api".to_string(),
            Self::Future => "https://future.forem.com/api".to_string(),
            Self::Gg => "https://gg.forem.com/api".to_string(),
            Self::Music => "https://music.forem.com/api".to_string(),
            Self::Popcorn => "https://popcorn.forem.com/api".to_string(),
            Self::Design => "https://design.forem.com/api".to_string(),
            Self::Zeroday => "https://zeroday.forem.com/api".to_string(),
            Self::Golf => "https://golf.forem.com/api".to_string(),
            Self::Crypto => "https://crypto.forem.com/api".to_string(),
            Self::Parenting => "https://parenting.forem.com/api".to_string(),
            Self::Core => "https://core.forem.com/api".to_string(),
            Self::Maker => "https://maker.forem.com/api".to_string(),
            Self::Hmpljs => "https://hmpljs.forem.com/api".to_string(),
            Self::DumbDev => "https://dumb.dev.to/api".to_string(),
            Self::Custom { domain } => format!("https://{domain}/api"),
        }
    }

    /// Returns the display name for this Forem instance.
    #[must_use]
    pub fn display_name(&self) -> String {
        match self {
            Self::DevTo => "Dev.to".to_string(),
            Self::Vibe => "Vibe Forem".to_string(),
            Self::Open => "Open Forem".to_string(),
            Self::Future => "Future Forem".to_string(),
            Self::Gg => "GG Forem".to_string(),
            Self::Music => "Music Forem".to_string(),
            Self::Popcorn => "Popcorn Forem".to_string(),
            Self::Design => "Design Forem".to_string(),
            Self::Zeroday => "Zeroday Forem".to_string(),
            Self::Golf => "Golf Forem".to_string(),
            Self::Crypto => "Crypto Forem".to_string(),
            Self::Parenting => "Parenting Forem".to_string(),
            Self::Core => "Core Forem".to_string(),
            Self::Maker => "Maker Forem".to_string(),
            Self::Hmpljs => "HMPL.js Forem".to_string(),
            Self::DumbDev => "Dumb Dev".to_string(),
            Self::Custom { domain } => format!("Forem ({domain})"),
        }
    }

    /// Returns the short identifier for this instance (used in platform strings).
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::DevTo => "devto".to_string(),
            Self::Vibe => "vibe".to_string(),
            Self::Open => "open".to_string(),
            Self::Future => "future".to_string(),
            Self::Gg => "gg".to_string(),
            Self::Music => "music".to_string(),
            Self::Popcorn => "popcorn".to_string(),
            Self::Design => "design".to_string(),
            Self::Zeroday => "zeroday".to_string(),
            Self::Golf => "golf".to_string(),
            Self::Crypto => "crypto".to_string(),
            Self::Parenting => "parenting".to_string(),
            Self::Core => "core".to_string(),
            Self::Maker => "maker".to_string(),
            Self::Hmpljs => "hmpljs".to_string(),
            Self::DumbDev => "dumbdev".to_string(),
            Self::Custom { domain } => format!("custom:{domain}"),
        }
    }
}

impl fmt::Display for ForemInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ForemInstance {
    type Err = PullError;

    /// Parse a Forem instance from a string.
    ///
    /// Supported formats:
    /// - "devto" or "dev.to" -> DevTo
    /// - "vibe" -> Vibe
    /// - "open" -> Open
    /// - ... (other known instances)
    /// - "custom:example.com" -> Custom { domain: "example.com" }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();

        // Check for custom instance first
        if let Some(domain) = lower.strip_prefix("custom:") {
            if domain.is_empty() {
                return Err(PullError::UnsupportedPlatform(
                    "Custom Forem instance requires a domain".to_string(),
                ));
            }
            return Ok(Self::Custom {
                domain: domain.to_string(),
            });
        }

        match lower.as_str() {
            "devto" | "dev.to" | "dev" => Ok(Self::DevTo),
            "vibe" | "vibe.forem" | "vibe.forem.com" | "vibeforem" => Ok(Self::Vibe),
            "open" | "open.forem" | "open.forem.com" => Ok(Self::Open),
            "future" | "future.forem" | "future.forem.com" => Ok(Self::Future),
            "gg" | "gg.forem" | "gg.forem.com" => Ok(Self::Gg),
            "music" | "music.forem" | "music.forem.com" => Ok(Self::Music),
            "popcorn" | "popcorn.forem" | "popcorn.forem.com" => Ok(Self::Popcorn),
            "design" | "design.forem" | "design.forem.com" => Ok(Self::Design),
            "zeroday" | "zeroday.forem" | "zeroday.forem.com" => Ok(Self::Zeroday),
            "golf" | "golf.forem" | "golf.forem.com" => Ok(Self::Golf),
            "crypto" | "crypto.forem" | "crypto.forem.com" => Ok(Self::Crypto),
            "parenting" | "parenting.forem" | "parenting.forem.com" => Ok(Self::Parenting),
            "core" | "core.forem" | "core.forem.com" => Ok(Self::Core),
            "maker" | "maker.forem" | "maker.forem.com" => Ok(Self::Maker),
            "hmpljs" | "hmpljs.forem" | "hmpljs.forem.com" => Ok(Self::Hmpljs),
            "dumbdev" | "dumb.dev" | "dumb.dev.to" => Ok(Self::DumbDev),
            _ => Err(PullError::UnsupportedPlatform(format!(
                "Unknown Forem instance: {s}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_url_devto() {
        assert_eq!(ForemInstance::DevTo.base_url(), "https://dev.to/api");
    }

    #[test]
    fn test_base_url_vibe() {
        assert_eq!(ForemInstance::Vibe.base_url(), "https://vibe.forem.com/api");
    }

    #[test]
    fn test_base_url_custom() {
        let custom = ForemInstance::Custom {
            domain: "my-forem.example.com".to_string(),
        };
        assert_eq!(custom.base_url(), "https://my-forem.example.com/api");
    }

    #[test]
    fn test_from_str_devto_variations() {
        assert_eq!(
            "devto".parse::<ForemInstance>().unwrap(),
            ForemInstance::DevTo
        );
        assert_eq!(
            "dev.to".parse::<ForemInstance>().unwrap(),
            ForemInstance::DevTo
        );
        assert_eq!(
            "dev".parse::<ForemInstance>().unwrap(),
            ForemInstance::DevTo
        );
        assert_eq!(
            "DEVTO".parse::<ForemInstance>().unwrap(),
            ForemInstance::DevTo
        );
    }

    #[test]
    fn test_from_str_vibe_variations() {
        assert_eq!(
            "vibe".parse::<ForemInstance>().unwrap(),
            ForemInstance::Vibe
        );
        assert_eq!(
            "vibeforem".parse::<ForemInstance>().unwrap(),
            ForemInstance::Vibe
        );
        assert_eq!(
            "vibe.forem".parse::<ForemInstance>().unwrap(),
            ForemInstance::Vibe
        );
        assert_eq!(
            "vibe.forem.com".parse::<ForemInstance>().unwrap(),
            ForemInstance::Vibe
        );
    }

    #[test]
    fn test_from_str_all_instances() {
        assert_eq!(
            "open".parse::<ForemInstance>().unwrap(),
            ForemInstance::Open
        );
        assert_eq!(
            "future".parse::<ForemInstance>().unwrap(),
            ForemInstance::Future
        );
        assert_eq!("gg".parse::<ForemInstance>().unwrap(), ForemInstance::Gg);
        assert_eq!(
            "music".parse::<ForemInstance>().unwrap(),
            ForemInstance::Music
        );
        assert_eq!(
            "popcorn".parse::<ForemInstance>().unwrap(),
            ForemInstance::Popcorn
        );
        assert_eq!(
            "design".parse::<ForemInstance>().unwrap(),
            ForemInstance::Design
        );
        assert_eq!(
            "zeroday".parse::<ForemInstance>().unwrap(),
            ForemInstance::Zeroday
        );
        assert_eq!(
            "golf".parse::<ForemInstance>().unwrap(),
            ForemInstance::Golf
        );
        assert_eq!(
            "crypto".parse::<ForemInstance>().unwrap(),
            ForemInstance::Crypto
        );
        assert_eq!(
            "parenting".parse::<ForemInstance>().unwrap(),
            ForemInstance::Parenting
        );
        assert_eq!(
            "core".parse::<ForemInstance>().unwrap(),
            ForemInstance::Core
        );
        assert_eq!(
            "maker".parse::<ForemInstance>().unwrap(),
            ForemInstance::Maker
        );
        assert_eq!(
            "hmpljs".parse::<ForemInstance>().unwrap(),
            ForemInstance::Hmpljs
        );
        assert_eq!(
            "dumbdev".parse::<ForemInstance>().unwrap(),
            ForemInstance::DumbDev
        );
    }

    #[test]
    fn test_from_str_custom() {
        let result = "custom:my-community.forem.com"
            .parse::<ForemInstance>()
            .unwrap();
        assert_eq!(
            result,
            ForemInstance::Custom {
                domain: "my-community.forem.com".to_string()
            }
        );
    }

    #[test]
    fn test_from_str_custom_empty_domain() {
        let result = "custom:".parse::<ForemInstance>();
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_unknown() {
        let result = "unknown".parse::<ForemInstance>();
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(ForemInstance::DevTo.to_string(), "devto");
        assert_eq!(ForemInstance::Vibe.to_string(), "vibe");
        assert_eq!(
            ForemInstance::Custom {
                domain: "example.com".to_string()
            }
            .to_string(),
            "custom:example.com"
        );
    }

    #[test]
    fn test_display_name() {
        assert_eq!(ForemInstance::DevTo.display_name(), "Dev.to");
        assert_eq!(ForemInstance::Vibe.display_name(), "Vibe Forem");
        assert_eq!(
            ForemInstance::Custom {
                domain: "example.com".to_string()
            }
            .display_name(),
            "Forem (example.com)"
        );
    }
}
