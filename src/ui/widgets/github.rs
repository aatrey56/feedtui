use crate::config::GithubConfig;
use crate::feeds::github::GithubFetcher;
use crate::feeds::{FeedData, FeedFetcher, GithubNotification};
use crate::ui::widgets::FeedWidget;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub struct GithubWidget {
    config: GithubConfig,
    notifications: Vec<GithubNotification>,
    loading: bool,
    error: Option<String>,
    scroll_state: ListState,
    selected: bool,
}

impl GithubWidget {
    pub fn new(config: GithubConfig) -> Self {
        let mut scroll_state = ListState::default();
        scroll_state.select(Some(0));

        Self {
            config,
            notifications: Vec::new(),
            loading: true,
            error: None,
            scroll_state,
            selected: false,
        }
    }
}

impl FeedWidget for GithubWidget {
    fn id(&self) -> String {
        format!(
            "github-{}-{}",
            self.config.position.row, self.config.position.col
        )
    }

    fn title(&self) -> &str {
        &self.config.title
    }

    fn position(&self) -> (usize, usize) {
        (self.config.position.row, self.config.position.col)
    }

    fn render(&self, frame: &mut Frame, area: Rect, selected: bool) {
        let border_style = if selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let unread_count = self.notifications.iter().filter(|n| n.unread).count();
        let title = if unread_count > 0 {
            format!(" {} ({} unread) ", self.config.title, unread_count)
        } else {
            format!(" {} ", self.config.title)
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        if self.loading && self.notifications.is_empty() {
            let loading_text = List::new(vec![ListItem::new("Loading...")]).block(block);
            frame.render_widget(loading_text, area);
            return;
        }

        if let Some(ref error) = self.error {
            let error_text =
                List::new(vec![ListItem::new(format!("Error: {}", error))]).block(block);
            frame.render_widget(error_text, area);
            return;
        }

        if self.notifications.is_empty() {
            let empty_text = List::new(vec![ListItem::new("No notifications")]).block(block);
            frame.render_widget(empty_text, area);
            return;
        }

        let items: Vec<ListItem> = self
            .notifications
            .iter()
            .enumerate()
            .map(|(i, notif)| {
                let unread_indicator = if notif.unread { "● " } else { "○ " };
                let title_line = Line::from(vec![
                    Span::styled(
                        format!("{}{} ", unread_indicator, i + 1),
                        if notif.unread {
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::DarkGray)
                        },
                    ),
                    Span::styled(&notif.title, Style::default().fg(Color::White)),
                ]);

                let meta_line = Line::from(vec![
                    Span::styled(
                        format!("   {} | ", notif.repository),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled(
                        format!("{} | ", notif.notification_type),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(&notif.reason, Style::default().fg(Color::DarkGray)),
                ]);

                ListItem::new(vec![title_line, meta_line])
            })
            .collect();

        let list = List::new(items).block(block).highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

        let mut state = self.scroll_state.clone();
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn update_data(&mut self, data: FeedData) {
        self.loading = false;
        match data {
            FeedData::Github(notifications) => {
                self.notifications = notifications;
                self.error = None;
            }
            FeedData::Error(e) => {
                self.error = Some(e);
            }
            FeedData::Loading => {
                self.loading = true;
            }
            _ => {}
        }
    }

    fn create_fetcher(&self) -> Box<dyn FeedFetcher> {
        Box::new(GithubFetcher::new(
            self.config.token.clone(),
            self.config.max_notifications,
        ))
    }

    fn scroll_up(&mut self) {
        if let Some(selected) = self.scroll_state.selected() {
            if selected > 0 {
                self.scroll_state.select(Some(selected - 1));
            }
        }
    }

    fn scroll_down(&mut self) {
        if let Some(selected) = self.scroll_state.selected() {
            if selected < self.notifications.len().saturating_sub(1) {
                self.scroll_state.select(Some(selected + 1));
            }
        }
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
