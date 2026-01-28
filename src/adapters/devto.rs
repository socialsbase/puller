use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use serde::Deserialize;
use url::Url;

use super::{ArticleMetadata, PullOptions, Puller};
use crate::article::PulledArticle;
use crate::error::{PullError, Result};
use crate::platform::Platform;

const DEVTO_API_BASE: &str = "https://dev.to/api";
const PER_PAGE: u32 = 100;

#[derive(Debug, Deserialize)]
struct DevToArticleListItem {
    id: u64,
    title: String,
    published_at: Option<DateTime<Utc>>,
    url: String,
    published: bool,
}

#[derive(Debug, Deserialize)]
struct DevToArticle {
    id: u64,
    title: String,
    body_markdown: String,
    published_at: Option<DateTime<Utc>>,
    url: String,
    tags: Vec<String>,
    #[serde(default)]
    series: Option<DevToSeries>,
    canonical_url: Option<String>,
    #[serde(default = "default_published")]
    published: bool,
}

fn default_published() -> bool {
    true
}

#[derive(Debug, Deserialize)]
struct DevToSeries {
    name: String,
}

pub struct DevToPuller {
    client: reqwest::Client,
    api_key: String,
}

impl DevToPuller {
    pub fn new(api_key: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.forem.api-v1+json"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("puller/0.1.0"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client, api_key })
    }

    async fn fetch_page(&self, page: u32) -> Result<Vec<DevToArticleListItem>> {
        let url = format!(
            "{}/articles/me?page={}&per_page={}",
            DEVTO_API_BASE, page, PER_PAGE
        );

        let response = self
            .client
            .get(&url)
            .header("api-key", &self.api_key)
            .send()
            .await?;

        if response.status() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);
            return Err(PullError::RateLimited(retry_after));
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(PullError::Api(format!(
                "Dev.to API returned {}: {}",
                status, body
            )));
        }

        Ok(response.json().await?)
    }
}

#[async_trait]
impl Puller for DevToPuller {
    fn platform(&self) -> Platform {
        Platform::DevTo
    }

    async fn list_articles(&self, options: &PullOptions) -> Result<Vec<ArticleMetadata>> {
        let mut all_articles = Vec::new();
        let mut page = 1;

        loop {
            let articles = self.fetch_page(page).await?;
            let count = articles.len();

            for article in articles {
                // Filter by date if specified
                if let Some(since) = options.since {
                    if let Some(published_at) = article.published_at {
                        if published_at.date_naive() < since {
                            continue;
                        }
                    }
                }

                // Filter drafts unless requested
                if !article.published && !options.include_drafts {
                    continue;
                }

                all_articles.push(ArticleMetadata {
                    id: article.id.to_string(),
                    platform: Platform::DevTo,
                    title: article.title,
                    published_at: article.published_at,
                    url: Url::parse(&article.url).ok(),
                    is_draft: !article.published,
                });
            }

            if count < PER_PAGE as usize {
                break;
            }
            page += 1;
        }

        Ok(all_articles)
    }

    async fn fetch_article(&self, id: &str) -> Result<PulledArticle> {
        let url = format!("{}/articles/{}", DEVTO_API_BASE, id);

        let response = self
            .client
            .get(&url)
            .header("api-key", &self.api_key)
            .send()
            .await?;

        if response.status() == 404 {
            return Err(PullError::NotFound(id.to_string()));
        }

        if response.status() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);
            return Err(PullError::RateLimited(retry_after));
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(PullError::Api(format!(
                "Dev.to API returned {}: {}",
                status, body
            )));
        }

        let article: DevToArticle = response.json().await?;

        Ok(PulledArticle {
            platform_id: article.id.to_string(),
            platform: Platform::DevTo,
            title: article.title,
            body_markdown: article.body_markdown,
            published_at: article.published_at,
            url: Url::parse(&article.url).ok(),
            tags: article.tags,
            series: article.series.map(|s| s.name),
            canonical_url: article.canonical_url.and_then(|u| Url::parse(&u).ok()),
            is_draft: !article.published,
        })
    }
}
