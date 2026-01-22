use super::{FeedData, FeedFetcher, GithubNotification};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

const GITHUB_API_BASE: &str = "https://api.github.com";

pub struct GithubFetcher {
    token: String,
    max_notifications: usize,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct GithubApiNotification {
    id: String,
    subject: Subject,
    repository: Repository,
    unread: bool,
    updated_at: String,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct Subject {
    title: String,
    #[serde(rename = "type")]
    notification_type: String,
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Repository {
    full_name: String,
}

impl GithubFetcher {
    pub fn new(token: String, max_notifications: usize) -> Self {
        Self {
            token,
            max_notifications,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl FeedFetcher for GithubFetcher {
    async fn fetch(&self) -> Result<FeedData> {
        let url = format!("{}/notifications", GITHUB_API_BASE);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "feedtui")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("GitHub API error: {}", response.status()));
        }

        let api_notifications: Vec<GithubApiNotification> = response.json().await?;

        let notifications: Vec<GithubNotification> = api_notifications
            .into_iter()
            .take(self.max_notifications)
            .map(|n| GithubNotification {
                id: n.id,
                title: n.subject.title,
                notification_type: n.subject.notification_type,
                repository: n.repository.full_name,
                url: n.subject.url.unwrap_or_else(|| "N/A".to_string()),
                unread: n.unread,
                updated_at: n.updated_at,
                reason: n.reason,
            })
            .collect();

        Ok(FeedData::Github(notifications))
    }
}
