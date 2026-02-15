#[derive(Debug, Clone)]
pub struct Tweet {
    #[allow(dead_code)] // Preserved for future use (e.g., threading, caching)
    pub id: String,
    pub author: String,
    pub text: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TwitterMessage {
    pub widget_id: String,
    pub data: TwitterData,
}

#[derive(Debug, Clone)]
pub enum TwitterData {
    TweetPosted(String),
    ReplyPosted(String),
    SearchResults(Vec<Tweet>),
    Mentions(Vec<Tweet>),
    TweetDetail(String),
    Error(String),
}
