pub mod devto;
pub mod vibe_forem;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use url::Url;

use crate::article::PulledArticle;
use crate::error::Result;
use crate::platform::Platform;

#[derive(Debug, Clone, Default)]
pub struct PullOptions {
    pub since: Option<NaiveDate>,
    pub include_drafts: bool,
}

#[derive(Debug, Clone)]
pub struct ArticleMetadata {
    pub id: String,
    pub platform: Platform,
    pub title: String,
    pub published_at: Option<DateTime<Utc>>,
    pub url: Option<Url>,
    pub is_draft: bool,
}

impl ArticleMetadata {
    pub fn platform_id(&self) -> String {
        format!("{}:{}", self.platform, self.id)
    }
}

#[async_trait]
pub trait Puller: Send + Sync {
    fn platform(&self) -> Platform;
    async fn list_articles(&self, options: &PullOptions) -> Result<Vec<ArticleMetadata>>;
    async fn fetch_article(&self, id: &str) -> Result<PulledArticle>;
}
