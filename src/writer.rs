use std::path::Path;

use clap::ValueEnum;

use crate::article::PulledArticle;
use crate::error::Result;
use crate::state::PullState;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum FolderStructure {
    #[default]
    Platform, // content/devto/article.md
    Flat, // content/article.md
}

pub struct Writer<'a> {
    output_dir: &'a Path,
    dry_run: bool,
    structure: FolderStructure,
}

impl<'a> Writer<'a> {
    pub fn new(output_dir: &'a Path, dry_run: bool, structure: FolderStructure) -> Self {
        Self {
            output_dir,
            dry_run,
            structure,
        }
    }

    pub fn write_article(&self, article: &PulledArticle, state: &mut PullState) -> Result<String> {
        let filename = article.generate_filename();
        let (filepath, relative_path) = match self.structure {
            FolderStructure::Flat => (self.output_dir.join(&filename), filename),
            FolderStructure::Platform => {
                let platform_str = article.platform.to_string();
                let platform_dir = self.output_dir.join(&platform_str);
                (
                    platform_dir.join(&filename),
                    format!("{platform_str}/{filename}"),
                )
            }
        };

        if !self.dry_run {
            // Create subdirectory if needed
            if let Some(parent) = filepath.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let content = article.to_markdown()?;
            std::fs::write(&filepath, content)?;

            let platform_id = format!("{}:{}", article.platform, article.platform_id);
            state.mark_pulled(platform_id, relative_path.clone());
        }

        Ok(relative_path)
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
    fn test_write_article_flat() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let writer = Writer::new(dir.path(), false, FolderStructure::Flat);
        let mut state = PullState::default();

        let article = PulledArticle {
            platform_id: "123".to_string(),
            platform: Platform::DevTo,
            title: "Test Article".to_string(),
            body_markdown: "Hello, world!".to_string(),
            published_at: Some("2024-03-15T10:00:00Z".parse()?),
            url: Some("https://dev.to/user/test-article".parse()?),
            tags: vec!["rust".to_string()],
            series: None,
            canonical_url: None,
            is_draft: false,
        };

        let relative_path = writer.write_article(&article, &mut state)?;
        assert_eq!(relative_path, "2024-03-15-test-article.md");

        let filepath = dir.path().join(&relative_path);
        assert!(filepath.exists());

        let content = std::fs::read_to_string(filepath)?;
        assert!(content.contains("title: Test Article"));
        assert!(content.contains("Hello, world!"));
        assert!(content.contains("# Platform ID: devto:123"));
        Ok(())
    }

    #[test]
    fn test_write_article_platform() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let writer = Writer::new(dir.path(), false, FolderStructure::Platform);
        let mut state = PullState::default();

        let article = PulledArticle {
            platform_id: "123".to_string(),
            platform: Platform::DevTo,
            title: "Test Article".to_string(),
            body_markdown: "Hello, world!".to_string(),
            published_at: Some("2024-03-15T10:00:00Z".parse()?),
            url: Some("https://dev.to/user/test-article".parse()?),
            tags: vec!["rust".to_string()],
            series: None,
            canonical_url: None,
            is_draft: false,
        };

        let relative_path = writer.write_article(&article, &mut state)?;
        assert_eq!(relative_path, "devto/2024-03-15-test-article.md");

        // Check the file exists in the platform subdirectory
        let filepath = dir.path().join(&relative_path);
        assert!(filepath.exists());

        // Check the subdirectory was created
        let platform_dir = dir.path().join("devto");
        assert!(platform_dir.exists());
        assert!(platform_dir.is_dir());

        let content = std::fs::read_to_string(filepath)?;
        assert!(content.contains("title: Test Article"));
        assert!(content.contains("Hello, world!"));
        assert!(content.contains("# Platform ID: devto:123"));
        Ok(())
    }

    #[test]
    fn test_dry_run_does_not_write() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let writer = Writer::new(dir.path(), true, FolderStructure::Flat);
        let mut state = PullState::default();

        let article = PulledArticle {
            platform_id: "123".to_string(),
            platform: Platform::DevTo,
            title: "Test Article".to_string(),
            body_markdown: "Hello, world!".to_string(),
            published_at: Some("2024-03-15T10:00:00Z".parse()?),
            url: None,
            tags: vec![],
            series: None,
            canonical_url: None,
            is_draft: false,
        };

        let relative_path = writer.write_article(&article, &mut state)?;
        let filepath = dir.path().join(&relative_path);
        assert!(!filepath.exists());
        assert!(!state.is_pulled("devto:123"));
        Ok(())
    }
}
