use crate::twitter_message::Tweet;
use serde::Deserialize;

/// Bird CLI JSON output structure for a tweet
#[derive(Debug, Deserialize)]
struct BirdTweet {
    id: String,
    text: String,
    author: Option<BirdAuthor>,
    #[serde(rename = "authorId")]
    author_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BirdAuthor {
    username: String,
    #[allow(dead_code)]
    name: Option<String>,
}

/// Parse Bird CLI `--json` output into Tweet structs.
/// Bird outputs a JSON array of tweet objects.
pub fn parse_json_tweets(output: &str) -> Vec<Tweet> {
    let trimmed = output.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    match serde_json::from_str::<Vec<BirdTweet>>(trimmed) {
        Ok(bird_tweets) => bird_tweets
            .into_iter()
            .map(|bt| {
                let username = bt
                    .author
                    .as_ref()
                    .map(|a| a.username.clone())
                    .unwrap_or_else(|| bt.author_id.clone().unwrap_or_else(|| "unknown".into()));
                let url = format!("https://x.com/{}/status/{}", username, bt.id);
                Tweet {
                    id: bt.id,
                    author: username,
                    text: bt.text.replace('\n', " "),
                    url: Some(url),
                }
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_tweets() {
        let input = r#"[
            {
                "id": "123456",
                "text": "Hello world",
                "createdAt": "Mon Feb 16 12:00:00 +0000 2026",
                "author": {
                    "username": "testuser",
                    "name": "Test User"
                },
                "authorId": "789"
            },
            {
                "id": "654321",
                "text": "Another tweet\nwith newlines",
                "author": {
                    "username": "other",
                    "name": "Other User"
                }
            }
        ]"#;
        let tweets = parse_json_tweets(input);
        assert_eq!(tweets.len(), 2);
        assert_eq!(tweets[0].id, "123456");
        assert_eq!(tweets[0].author, "testuser");
        assert_eq!(tweets[0].text, "Hello world");
        assert_eq!(
            tweets[0].url,
            Some("https://x.com/testuser/status/123456".to_string())
        );
        assert_eq!(tweets[1].text, "Another tweet with newlines");
    }

    #[test]
    fn test_parse_empty_array() {
        let tweets = parse_json_tweets("[]");
        assert!(tweets.is_empty());
    }

    #[test]
    fn test_parse_empty_string() {
        let tweets = parse_json_tweets("");
        assert!(tweets.is_empty());
    }

    #[test]
    fn test_parse_invalid_json() {
        let tweets = parse_json_tweets("not json at all");
        assert!(tweets.is_empty());
    }
}
