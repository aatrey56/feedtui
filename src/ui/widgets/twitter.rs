use crate::config::TwitterConfig;
use crate::feeds::{FeedData, FeedFetcher};
use crate::ui::widgets::FeedWidget;
use async_trait::async_trait;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::any::Any;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct TwitterWidget {
    id: String,
    title: String,
    position: (usize, usize),
    selected: bool,
    tweets: Vec<Tweet>,
    selected_index: usize,
    list_state: ListState,
    mode: TwitterMode,
    compose_text: String,
    search_query: String,
    detail_view: Option<TweetDetail>,
    status_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum TwitterMode {
    Normal,
    Compose,
    Reply,
    Search,
}

#[derive(Debug, Clone)]
struct Tweet {
    id: String,
    author: String,
    text: String,
    url: Option<String>,
}

#[derive(Debug, Clone)]
struct TweetDetail {
    content: String,
}

impl TwitterWidget {
    pub fn new(config: TwitterConfig) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            id: format!("twitter-{}-{}", config.position.row, config.position.col),
            title: config.title,
            position: (config.position.row, config.position.col),
            selected: false,
            tweets: Vec::new(),
            selected_index: 0,
            list_state,
            mode: TwitterMode::Normal,
            compose_text: String::new(),
            search_query: String::new(),
            detail_view: None,
            status_message: None,
        }
    }

    pub fn open_compose(&mut self) {
        self.mode = TwitterMode::Compose;
        self.compose_text.clear();
    }

    pub fn open_reply(&mut self) {
        if !self.tweets.is_empty() {
            self.mode = TwitterMode::Reply;
            self.compose_text.clear();
        }
    }

    pub fn open_search(&mut self) {
        self.mode = TwitterMode::Search;
        self.search_query.clear();
    }

    pub fn close_modal(&mut self) {
        self.mode = TwitterMode::Normal;
        self.compose_text.clear();
        self.search_query.clear();
    }

    pub fn add_char(&mut self, c: char) {
        match self.mode {
            TwitterMode::Compose | TwitterMode::Reply => {
                self.compose_text.push(c);
            }
            TwitterMode::Search => {
                self.search_query.push(c);
            }
            _ => {}
        }
    }

    pub fn delete_char(&mut self) {
        match self.mode {
            TwitterMode::Compose | TwitterMode::Reply => {
                self.compose_text.pop();
            }
            TwitterMode::Search => {
                self.search_query.pop();
            }
            _ => {}
        }
    }

    pub async fn submit_tweet(&mut self) {
        if self.compose_text.is_empty() {
            return;
        }

        match self.execute_bird_command(&["tweet", &self.compose_text]).await {
            Ok(output) => {
                self.status_message = Some(format!("Tweet posted: {}", output));
                self.close_modal();
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    pub async fn submit_reply(&mut self) {
        if self.compose_text.is_empty() || self.tweets.is_empty() {
            return;
        }

        if let Some(tweet) = self.tweets.get(self.selected_index) {
            if let Some(url) = &tweet.url {
                match self
                    .execute_bird_command(&["reply", url, &self.compose_text])
                    .await
                {
                    Ok(output) => {
                        self.status_message = Some(format!("Reply posted: {}", output));
                        self.close_modal();
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }
            }
        }
    }

    pub async fn submit_search(&mut self) {
        if self.search_query.is_empty() {
            return;
        }

        match self
            .execute_bird_command(&["search", &self.search_query, "-n", "5"])
            .await
        {
            Ok(output) => {
                self.parse_search_results(&output);
                self.close_modal();
            }
            Err(e) => {
                self.status_message = Some(format!("Search error: {}", e));
            }
        }
    }

    pub async fn load_mentions(&mut self) {
        match self.execute_bird_command(&["mentions", "-n", "5"]).await {
            Ok(output) => {
                self.parse_mentions(&output);
            }
            Err(e) => {
                self.status_message = Some(format!("Mentions error: {}", e));
            }
        }
    }

    pub async fn read_selected_tweet(&mut self) {
        if self.tweets.is_empty() {
            return;
        }

        if let Some(tweet) = self.tweets.get(self.selected_index) {
            if let Some(url) = &tweet.url {
                match self.execute_bird_command(&["read", url]).await {
                    Ok(output) => {
                        self.detail_view = Some(TweetDetail { content: output });
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Read error: {}", e));
                    }
                }
            }
        }
    }

    pub fn close_detail_view(&mut self) {
        self.detail_view = None;
    }

    async fn execute_bird_command(&self, args: &[&str]) -> anyhow::Result<String> {
        // Check if bird is installed
        let bird_check = Command::new("which")
            .arg("bird")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;

        if bird_check.is_err() || !bird_check.unwrap().success() {
            return Err(anyhow::anyhow!(
                "Bird CLI not found. Install with: bun install -g bird-cli"
            ));
        }

        // Check for environment variables
        if std::env::var("CT0").is_err() || std::env::var("AUTH_TOKEN").is_err() {
            return Err(anyhow::anyhow!(
                "Missing CT0 or AUTH_TOKEN environment variables"
            ));
        }

        // Execute bird command
        let output = Command::new("bird")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow::anyhow!(
                "Bird command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn parse_search_results(&mut self, output: &str) {
        // Simple parsing - in production, you'd want more robust parsing
        self.tweets.clear();
        for (idx, line) in output.lines().enumerate() {
            if !line.is_empty() {
                self.tweets.push(Tweet {
                    id: format!("tweet-{}", idx),
                    author: "Unknown".to_string(),
                    text: line.to_string(),
                    url: None,
                });
            }
        }
    }

    fn parse_mentions(&mut self, output: &str) {
        // Simple parsing - in production, you'd want more robust parsing
        self.tweets.clear();
        for (idx, line) in output.lines().enumerate() {
            if !line.is_empty() {
                self.tweets.push(Tweet {
                    id: format!("mention-{}", idx),
                    author: "Unknown".to_string(),
                    text: line.to_string(),
                    url: None,
                });
            }
        }
    }

    pub fn is_modal_open(&self) -> bool {
        self.mode != TwitterMode::Normal || self.detail_view.is_some()
    }
}

struct TwitterFetcher;

#[async_trait]
impl FeedFetcher for TwitterFetcher {
    async fn fetch(&self) -> anyhow::Result<FeedData> {
        // Twitter widget doesn't auto-fetch
        Ok(FeedData::Empty)
    }
}

impl FeedWidget for TwitterWidget {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn position(&self) -> (usize, usize) {
        self.position
    }

    fn render(&self, frame: &mut Frame, area: Rect, selected: bool) {
        let border_style = if selected {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::Gray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(self.title.as_str());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Render tweet list
        if self.tweets.is_empty() {
            let help_text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Twitter/X Feed",
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from("Keybindings:"),
                Line::from("  t - Compose tweet"),
                Line::from("  r - Reply to selected"),
                Line::from("  / - Search"),
                Line::from("  m - Load mentions"),
                Line::from("  Enter - Read tweet/thread"),
                Line::from(""),
                Line::from(Span::styled(
                    "Note: Requires Bird CLI and auth",
                    Style::default().fg(Color::Yellow),
                )),
            ];
            let paragraph = Paragraph::new(help_text).alignment(Alignment::Center);
            frame.render_widget(paragraph, inner);
        } else {
            let items: Vec<ListItem> = self
                .tweets
                .iter()
                .enumerate()
                .map(|(idx, tweet)| {
                    let style = if idx == self.selected_index {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(&tweet.author, style.add_modifier(Modifier::BOLD)),
                        Span::raw(": "),
                        Span::styled(&tweet.text, style),
                    ]))
                })
                .collect();

            let list = List::new(items).block(Block::default());
            frame.render_widget(list, inner);
        }

        // Render modals if open
        if self.mode == TwitterMode::Compose {
            self.render_compose_modal(frame, area);
        } else if self.mode == TwitterMode::Reply {
            self.render_reply_modal(frame, area);
        } else if self.mode == TwitterMode::Search {
            self.render_search_modal(frame, area);
        }

        // Render detail view if open
        if let Some(detail) = &self.detail_view {
            self.render_detail_view(frame, area, detail);
        }

        // Render status message if present
        if let Some(msg) = &self.status_message {
            self.render_status(frame, area, msg);
        }
    }

    fn update_data(&mut self, _data: FeedData) {
        // Twitter widget doesn't use standard feed data
    }

    fn create_fetcher(&self) -> Box<dyn FeedFetcher> {
        Box::new(TwitterFetcher)
    }

    fn scroll_up(&mut self) {
        if !self.tweets.is_empty() && self.selected_index > 0 {
            self.selected_index -= 1;
            self.list_state.select(Some(self.selected_index));
        }
    }

    fn scroll_down(&mut self) {
        if !self.tweets.is_empty() && self.selected_index < self.tweets.len() - 1 {
            self.selected_index += 1;
            self.list_state.select(Some(self.selected_index));
        }
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }

    fn get_selected_discussion_url(&self) -> Option<String> {
        self.tweets
            .get(self.selected_index)
            .and_then(|t| t.url.clone())
    }
}

impl TwitterWidget {
    fn render_compose_modal(&self, frame: &mut Frame, area: Rect) {
        let modal_area = self.center_rect(60, 30, area);
        frame.render_widget(Clear, modal_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Compose Tweet");

        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let text = vec![
            Line::from(""),
            Line::from(&self.compose_text),
            Line::from(""),
            Line::from(Span::styled(
                "Enter to post | Esc to cancel",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
        frame.render_widget(paragraph, inner);
    }

    fn render_reply_modal(&self, frame: &mut Frame, area: Rect) {
        let modal_area = self.center_rect(60, 30, area);
        frame.render_widget(Clear, modal_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Reply to Tweet");

        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let text = vec![
            Line::from(""),
            Line::from(&self.compose_text),
            Line::from(""),
            Line::from(Span::styled(
                "Enter to post | Esc to cancel",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
        frame.render_widget(paragraph, inner);
    }

    fn render_search_modal(&self, frame: &mut Frame, area: Rect) {
        let modal_area = self.center_rect(60, 20, area);
        frame.render_widget(Clear, modal_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Search Twitter");

        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let text = vec![
            Line::from(""),
            Line::from(format!("Query: {}", self.search_query)),
            Line::from(""),
            Line::from(Span::styled(
                "Enter to search | Esc to cancel",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, inner);
    }

    fn render_detail_view(&self, frame: &mut Frame, area: Rect, detail: &TweetDetail) {
        let modal_area = self.center_rect(80, 80, area);
        frame.render_widget(Clear, modal_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Tweet Detail");

        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let paragraph = Paragraph::new(detail.content.as_str())
            .wrap(Wrap { trim: false })
            .block(Block::default());
        frame.render_widget(paragraph, inner);
    }

    fn render_status(&self, frame: &mut Frame, area: Rect, message: &str) {
        let status_area = Rect::new(
            area.x + 2,
            area.y + area.height.saturating_sub(3),
            area.width.saturating_sub(4),
            3,
        );

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        let paragraph = Paragraph::new(message).block(block);
        frame.render_widget(Clear, status_area);
        frame.render_widget(paragraph, status_area);
    }

    fn center_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
