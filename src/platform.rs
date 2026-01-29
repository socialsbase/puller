use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::PullError;
use crate::forem::ForemInstance;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    /// Forem-based platforms (dev.to, vibe.forem.com, etc.)
    #[serde(untagged)]
    Forem(ForemInstance),
}

impl Platform {
    /// Returns the ForemInstance for this platform.
    #[must_use]
    pub fn as_forem(&self) -> &ForemInstance {
        match self {
            Platform::Forem(instance) => instance,
        }
    }

    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::Forem(instance) => {
                // For backward compatibility, DevTo displays as "devto"
                if *instance == ForemInstance::DevTo {
                    "devto".to_string()
                } else {
                    format!("forem:{}", instance)
                }
            }
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
        let lower = s.to_lowercase();

        // Backward compatibility: "devto", "dev.to", "dev" map directly to Forem(DevTo)
        // Also "vibeforem", "vibe.forem", "vibe" map to Forem(Vibe)
        match lower.as_str() {
            "devto" | "dev.to" | "dev" => Ok(Platform::Forem(ForemInstance::DevTo)),
            "vibeforem" | "vibe.forem" | "vibe" => Ok(Platform::Forem(ForemInstance::Vibe)),
            _ => {
                // Check for forem: prefix (e.g., "forem:vibe", "forem:custom:example.com")
                if let Some(instance_str) = lower.strip_prefix("forem:") {
                    let instance: ForemInstance = instance_str.parse()?;
                    Ok(Platform::Forem(instance))
                } else {
                    // Try to parse as a direct Forem instance name
                    let instance: ForemInstance = lower.parse()?;
                    Ok(Platform::Forem(instance))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_devto() {
        assert_eq!(Platform::Forem(ForemInstance::DevTo).to_string(), "devto");
    }

    #[test]
    fn test_display_vibe() {
        assert_eq!(
            Platform::Forem(ForemInstance::Vibe).to_string(),
            "forem:vibe"
        );
    }

    #[test]
    fn test_display_custom() {
        let platform = Platform::Forem(ForemInstance::Custom {
            domain: "example.com".to_string(),
        });
        assert_eq!(platform.to_string(), "forem:custom:example.com");
    }

    #[test]
    fn test_from_str_devto() {
        assert_eq!(
            "devto".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::DevTo)
        );
    }

    #[test]
    fn test_from_str_devto_alias() {
        assert_eq!(
            "dev.to".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::DevTo)
        );
        assert_eq!(
            "dev".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::DevTo)
        );
    }

    #[test]
    fn test_from_str_vibeforem_variations() {
        assert_eq!(
            "vibeforem".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::Vibe)
        );
        assert_eq!(
            "vibe.forem".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::Vibe)
        );
        assert_eq!(
            "vibe".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::Vibe)
        );
    }

    #[test]
    fn test_from_str_forem_prefix() {
        assert_eq!(
            "forem:vibe".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::Vibe)
        );
        assert_eq!(
            "forem:open".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::Open)
        );
    }

    #[test]
    fn test_from_str_forem_custom() {
        assert_eq!(
            "forem:custom:my-community.forem.com"
                .parse::<Platform>()
                .unwrap(),
            Platform::Forem(ForemInstance::Custom {
                domain: "my-community.forem.com".to_string()
            })
        );
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert_eq!(
            "DEVTO".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::DevTo)
        );
        assert_eq!(
            "DEV.TO".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::DevTo)
        );
        assert_eq!(
            "FOREM:VIBE".parse::<Platform>().unwrap(),
            Platform::Forem(ForemInstance::Vibe)
        );
    }

    #[test]
    fn test_from_str_unknown_platform() {
        let result = "facebook".parse::<Platform>();
        assert!(result.is_err());
    }

    #[test]
    fn test_platform_equality() {
        assert_eq!(
            Platform::Forem(ForemInstance::DevTo),
            Platform::Forem(ForemInstance::DevTo)
        );
        assert_ne!(
            Platform::Forem(ForemInstance::DevTo),
            Platform::Forem(ForemInstance::Vibe)
        );
    }

    #[test]
    fn test_platform_clone() {
        let p = Platform::Forem(ForemInstance::Vibe);
        let p_clone = p.clone();
        assert_eq!(p, p_clone);
    }

    #[test]
    fn test_as_forem() {
        let devto = Platform::Forem(ForemInstance::DevTo);
        assert_eq!(devto.as_forem(), &ForemInstance::DevTo);
    }
}
