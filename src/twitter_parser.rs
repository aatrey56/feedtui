use crate::twitter_message::Tweet;

/// Parse Bird CLI search results output into Tweet structs
/// Expected format:
/// @username: Tweet text
/// URL: https://twitter.com/...
/// ---
pub fn parse_search_results(output: &str) -> Vec<Tweet> {
    let mut tweets = Vec::new();
    let mut current_tweet: Option<(String, String, String)> = None; // (author, text, url)

    for line in output.lines() {
        let line = line.trim();

        if line.starts_with('@') {
            // Finalize previous tweet if exists
            if let Some((author, text, url)) = current_tweet.take() {
                if !url.is_empty() {
                    tweets.push(Tweet {
                        id: url.clone(),
                        author,
                        text,
                        url: Some(url),
                    });
                }
            }

            // Start new tweet
            if let Some((author, text)) = line.split_once(':') {
                current_tweet = Some((
                    author.trim_start_matches('@').to_string(),
                    text.trim().to_string(),
                    String::new(),
                ));
            }
        } else if line.starts_with("URL:") || line.starts_with("http") {
            if let Some((author, text, _)) = current_tweet.as_mut() {
                let url = line.trim_start_matches("URL:").trim();
                current_tweet = Some((author.clone(), text.clone(), url.to_string()));
            }
        } else if line == "---" || line.is_empty() {
            // Separator - finalize current tweet
            if let Some((author, text, url)) = current_tweet.take() {
                if !url.is_empty() {
                    tweets.push(Tweet {
                        id: url.clone(),
                        author,
                        text,
                        url: Some(url),
                    });
                }
            }
        } else if let Some((_, text, _)) = current_tweet.as_mut() {
            // Continuation of tweet text
            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str(line);
        }
    }

    // Finalize last tweet
    if let Some((author, text, url)) = current_tweet {
        if !url.is_empty() {
            tweets.push(Tweet {
                id: url.clone(),
                author,
                text,
                url: Some(url),
            });
        }
    }

    tweets
}

/// Parse Bird CLI mentions output (same format as search results)
pub fn parse_mentions(output: &str) -> Vec<Tweet> {
    parse_search_results(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_search_results() {
        let input = "@user1: This is a tweet\nURL: https://twitter.com/user1/status/123\n---\n@user2: Another tweet\nURL: https://twitter.com/user2/status/456";
        let tweets = parse_search_results(input);
        assert_eq!(tweets.len(), 2);
        assert_eq!(tweets[0].author, "user1");
        assert_eq!(tweets[0].text, "This is a tweet");
        assert_eq!(
            tweets[0].url,
            Some("https://twitter.com/user1/status/123".to_string())
        );
    }

    #[test]
    fn test_parse_multiline_tweet() {
        let input = "@user1: This is a long tweet\nthat spans multiple lines\nURL: https://twitter.com/user1/status/123";
        let tweets = parse_search_results(input);
        assert_eq!(tweets.len(), 1);
        assert_eq!(
            tweets[0].text,
            "This is a long tweet that spans multiple lines"
        );
    }
}
