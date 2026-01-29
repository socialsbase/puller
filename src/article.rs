use std::fmt::Write;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::Result;
use crate::platform::Platform;

#[derive(Debug, Clone)]
pub struct PulledArticle {
    pub platform_id: String,
    pub platform: Platform,
    pub title: String,
    pub body_markdown: String,
    pub published_at: Option<DateTime<Utc>>,
    #[allow(dead_code)] // Reserved for future use (e.g., verbose output)
    pub url: Option<Url>,
    pub tags: Vec<String>,
    pub series: Option<String>,
    pub canonical_url: Option<Url>,
    pub is_draft: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Frontmatter {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_at: Option<DateTime<Utc>>,
    status: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canonical_url: Option<Url>,
}

impl PulledArticle {
    fn to_frontmatter(&self) -> Frontmatter {
        Frontmatter {
            title: self.title.clone(),
            scheduled_at: self.published_at,
            status: if self.is_draft {
                "draft".to_string()
            } else {
                "publish".to_string()
            },
            tags: self.tags.clone(),
            series: self.series.clone(),
            canonical_url: self.canonical_url.clone(),
        }
    }

    pub fn to_markdown(&self) -> Result<String> {
        let frontmatter = self.to_frontmatter();
        let yaml = serde_yaml::to_string(&frontmatter)?;

        let mut output = String::new();
        output.push_str("---\n");
        output.push_str(&yaml);

        // Add platform ID comment for tracking
        writeln!(
            output,
            "# Platform ID: {}:{}",
            self.platform, self.platform_id
        )
        .expect("String write failed");

        output.push_str("---\n\n");
        output.push_str(&self.body_markdown);

        // Ensure file ends with newline
        if !output.ends_with('\n') {
            output.push('\n');
        }

        Ok(output)
    }

    pub fn generate_filename(&self) -> String {
        let date_prefix = self.published_at.map_or_else(
            || "draft".to_string(),
            |dt| dt.format("%Y-%m-%d").to_string(),
        );

        let slug = slugify(&self.title);
        format!("{date_prefix}-{slug}.md")
    }
}

fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forem::ForemInstance;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(
            slugify("Building CLI Tools in Rust"),
            "building-cli-tools-in-rust"
        );
        assert_eq!(slugify("What's New?"), "what-s-new");
        assert_eq!(slugify("Multiple   Spaces"), "multiple-spaces");
    }

    #[test]
    fn test_generate_filename() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let article = PulledArticle {
            platform_id: "123".to_string(),
            platform: Platform::Forem(ForemInstance::DevTo),
            title: "Building CLI Tools in Rust".to_string(),
            body_markdown: "Content".to_string(),
            published_at: Some("2024-03-15T10:00:00Z".parse()?),
            url: None,
            tags: vec![],
            series: None,
            canonical_url: None,
            is_draft: false,
        };

        assert_eq!(
            article.generate_filename(),
            "2024-03-15-building-cli-tools-in-rust.md"
        );
        Ok(())
    }

    #[test]
    fn test_generate_filename_draft() {
        let article = PulledArticle {
            platform_id: "123".to_string(),
            platform: Platform::Forem(ForemInstance::DevTo),
            title: "My Draft".to_string(),
            body_markdown: "Content".to_string(),
            published_at: None,
            url: None,
            tags: vec![],
            series: None,
            canonical_url: None,
            is_draft: true,
        };

        assert_eq!(article.generate_filename(), "draft-my-draft.md");
    }
}
