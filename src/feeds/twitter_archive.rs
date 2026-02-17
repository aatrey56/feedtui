use super::{FeedData, FeedFetcher, TwitterArchiveItem};
use anyhow::Result;
use async_trait::async_trait;

pub struct TwitterArchiveFetcher {
    archive_query: String,
    max_items: usize,
    client: reqwest::Client,
}

impl TwitterArchiveFetcher {
    pub fn new(archive_query: String, max_items: usize) -> Self {
        Self {
            archive_query,
            max_items,
            client: reqwest::Client::new(),
        }
    }

    /// Fetch capture records from the Wayback Machine CDX API.
    /// Returns JSON rows: [urlkey, timestamp, original, mimetype, statuscode, digest, length]
    async fn fetch_cdx_records(&self) -> Result<Vec<TwitterArchiveItem>> {
        // Ensure the query targets tweet URLs specifically (/status/*)
        // so we don't waste our limit on profile/media pages
        let query = if self.archive_query.contains("/status") {
            self.archive_query.clone()
        } else {
            let base = self
                .archive_query
                .trim_end_matches('*')
                .trim_end_matches('/');
            format!("{}/status/*", base)
        };

        let url = format!(
            "https://web.archive.org/cdx/search/cdx?url={}&output=json&limit={}&fl=timestamp,original,statuscode&filter=statuscode:200&collapse=urlkey",
            urlencoding::encode(&query),
            self.max_items + 1, // +1 because first row is the header
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "feedtui/1.0")
            .send()
            .await?;

        let body = response.text().await?;
        let rows: Vec<Vec<String>> = serde_json::from_str(&body)?;

        // First row is the header: ["timestamp", "original", "statuscode"]
        let items: Vec<TwitterArchiveItem> = rows
            .into_iter()
            .skip(1) // skip header row
            .filter_map(|row| {
                if row.len() < 3 {
                    return None;
                }

                let timestamp = &row[0];
                let original = &row[1];

                // Filter to only clean tweet URLs (contain /status/<id>)
                // Skip bare /status pages, URLs with query params or encoding artifacts
                if !original.contains("/status/") {
                    return None;
                }
                // Extract the part after /status/ and check it starts with a digit
                let after_status = original.split("/status/").nth(1).unwrap_or("");
                let tweet_id = after_status
                    .split(&['?', '%', '#', '"'][..])
                    .next()
                    .unwrap_or("");
                if tweet_id.is_empty()
                    || !tweet_id
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_ascii_digit())
                {
                    return None;
                }

                let archive_url = format!("https://web.archive.org/web/{}/{}", timestamp, original);

                // Parse timestamp (format: YYYYMMDDHHmmss) into readable date
                let date_display = format_wayback_timestamp(timestamp);

                // Extract author from URL pattern twitter.com/{author}/status/...
                let author = extract_author_from_url(original);

                Some(TwitterArchiveItem {
                    timestamp: timestamp.clone(),
                    original_url: original.clone(),
                    archive_url,
                    tweet_text: None,
                    author,
                    date_display,
                })
            })
            .take(self.max_items)
            .collect();

        Ok(items)
    }
}

/// Parse Wayback Machine timestamp (YYYYMMDDHHmmss) into a human-readable date.
fn format_wayback_timestamp(ts: &str) -> String {
    if ts.len() >= 8 {
        let year = &ts[0..4];
        let month = &ts[4..6];
        let day = &ts[6..8];
        let time = if ts.len() >= 12 {
            format!(" {}:{}", &ts[8..10], &ts[10..12])
        } else {
            String::new()
        };
        format!("{}-{}-{}{}", year, month, day, time)
    } else {
        ts.to_string()
    }
}

/// Extract the Twitter author handle from a URL like twitter.com/username/status/...
fn extract_author_from_url(url: &str) -> Option<String> {
    // Handle both http and https, with or without www
    let path = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    let path = path.strip_prefix("www.").unwrap_or(path);

    let path = path
        .strip_prefix("twitter.com/")
        .or_else(|| path.strip_prefix("x.com/"))?;

    let username = path.split('/').next()?;
    if username.is_empty() {
        None
    } else {
        Some(format!("@{}", username))
    }
}

#[async_trait]
impl FeedFetcher for TwitterArchiveFetcher {
    async fn fetch(&self) -> Result<FeedData> {
        let items = self.fetch_cdx_records().await?;
        Ok(FeedData::TwitterArchive(items))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_wayback_timestamp_full() {
        assert_eq!(
            format_wayback_timestamp("20230615143022"),
            "2023-06-15 14:30"
        );
    }

    #[test]
    fn test_format_wayback_timestamp_date_only() {
        assert_eq!(format_wayback_timestamp("20230615"), "2023-06-15");
    }

    #[test]
    fn test_format_wayback_timestamp_short() {
        assert_eq!(format_wayback_timestamp("2023"), "2023");
    }

    #[test]
    fn test_extract_author_https() {
        assert_eq!(
            extract_author_from_url("https://twitter.com/gethigher77/status/123456"),
            Some("@gethigher77".to_string())
        );
    }

    #[test]
    fn test_extract_author_http() {
        assert_eq!(
            extract_author_from_url("http://twitter.com/someuser/status/789"),
            Some("@someuser".to_string())
        );
    }

    #[test]
    fn test_extract_author_x_dot_com() {
        assert_eq!(
            extract_author_from_url("https://x.com/testuser/status/111"),
            Some("@testuser".to_string())
        );
    }

    #[test]
    fn test_extract_author_www() {
        assert_eq!(
            extract_author_from_url("https://www.twitter.com/handle/status/999"),
            Some("@handle".to_string())
        );
    }

    #[test]
    fn test_extract_author_no_match() {
        assert_eq!(extract_author_from_url("https://example.com/page"), None);
    }

    #[test]
    fn test_extract_author_empty_username() {
        assert_eq!(extract_author_from_url("https://twitter.com/"), None);
    }

    #[test]
    fn test_fetcher_new() {
        let fetcher = TwitterArchiveFetcher::new("twitter.com/test*".to_string(), 10);
        assert_eq!(fetcher.archive_query, "twitter.com/test*");
        assert_eq!(fetcher.max_items, 10);
    }
}
