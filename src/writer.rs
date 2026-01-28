use std::path::Path;

use crate::article::PulledArticle;
use crate::error::Result;
use crate::state::PullState;

pub struct Writer<'a> {
    output_dir: &'a Path,
    dry_run: bool,
}

impl<'a> Writer<'a> {
    pub fn new(output_dir: &'a Path, dry_run: bool) -> Self {
        Self {
            output_dir,
            dry_run,
        }
    }

    pub fn write_article(&self, article: &PulledArticle, state: &mut PullState) -> Result<String> {
        let filename = article.generate_filename();
        let filepath = self.output_dir.join(&filename);

        if !self.dry_run {
            let content = article.to_markdown()?;
            std::fs::write(&filepath, content)?;

            let platform_id = format!("{}:{}", article.platform, article.platform_id);
            state.mark_pulled(platform_id, filename.clone());
        }

        Ok(filename)
    }

    pub fn ensure_output_dir(&self) -> Result<()> {
        if !self.dry_run && !self.output_dir.exists() {
            std::fs::create_dir_all(self.output_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::Platform;
    use tempfile::TempDir;

    #[test]
    fn test_write_article() {
        let dir = TempDir::new().unwrap();
        let writer = Writer::new(dir.path(), false);
        let mut state = PullState::default();

        let article = PulledArticle {
            platform_id: "123".to_string(),
            platform: Platform::DevTo,
            title: "Test Article".to_string(),
            body_markdown: "Hello, world!".to_string(),
            published_at: Some("2024-03-15T10:00:00Z".parse().unwrap()),
            url: Some("https://dev.to/user/test-article".parse().unwrap()),
            tags: vec!["rust".to_string()],
            series: None,
            canonical_url: None,
            is_draft: false,
        };

        let filename = writer.write_article(&article, &mut state).unwrap();
        assert_eq!(filename, "2024-03-15-test-article.md");

        let filepath = dir.path().join(&filename);
        assert!(filepath.exists());

        let content = std::fs::read_to_string(filepath).unwrap();
        assert!(content.contains("title: Test Article"));
        assert!(content.contains("Hello, world!"));
        assert!(content.contains("# Platform ID: devto:123"));
    }

    #[test]
    fn test_dry_run_does_not_write() {
        let dir = TempDir::new().unwrap();
        let writer = Writer::new(dir.path(), true);
        let mut state = PullState::default();

        let article = PulledArticle {
            platform_id: "123".to_string(),
            platform: Platform::DevTo,
            title: "Test Article".to_string(),
            body_markdown: "Hello, world!".to_string(),
            published_at: Some("2024-03-15T10:00:00Z".parse().unwrap()),
            url: None,
            tags: vec![],
            series: None,
            canonical_url: None,
            is_draft: false,
        };

        let filename = writer.write_article(&article, &mut state).unwrap();
        let filepath = dir.path().join(&filename);
        assert!(!filepath.exists());
        assert!(!state.is_pulled("devto:123"));
    }
}
