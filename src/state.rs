use std::collections::HashMap;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::Result;

const STATE_FILENAME: &str = ".puller-state.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulledEntry {
    pub local_path: String,
    pub pulled_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PullState {
    pub pulled: HashMap<String, PulledEntry>,
}

impl PullState {
    pub fn load(output_dir: &Path) -> Result<Self> {
        let state_path = output_dir.join(STATE_FILENAME);
        if state_path.exists() {
            let content = std::fs::read_to_string(&state_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, output_dir: &Path) -> Result<()> {
        let state_path = output_dir.join(STATE_FILENAME);
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(state_path, content)?;
        Ok(())
    }

    pub fn is_pulled(&self, platform_id: &str) -> bool {
        self.pulled.contains_key(platform_id)
    }

    pub fn mark_pulled(&mut self, platform_id: String, local_path: String) {
        self.pulled.insert(
            platform_id,
            PulledEntry {
                local_path,
                pulled_at: Utc::now(),
            },
        );
    }

    pub fn get_local_path(&self, platform_id: &str) -> Option<&str> {
        self.pulled.get(platform_id).map(|e| e.local_path.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_state_roundtrip() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let mut state = PullState::default();
        state.mark_pulled(
            "devto:123".to_string(),
            "2024-03-15-test-article.md".to_string(),
        );

        state.save(dir.path())?;

        let loaded = PullState::load(dir.path())?;
        assert!(loaded.is_pulled("devto:123"));
        assert_eq!(
            loaded.get_local_path("devto:123"),
            Some("2024-03-15-test-article.md")
        );
        Ok(())
    }

    #[test]
    fn test_load_nonexistent() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let state = PullState::load(dir.path())?;
        assert!(state.pulled.is_empty());
        Ok(())
    }
}
