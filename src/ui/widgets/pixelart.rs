use crate::config::PixelArtConfig;
use crate::feeds::{FeedData, FeedFetcher};
use crate::ui::widgets::FeedWidget;
use async_trait::async_trait;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::any::Any;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PixelArtWidget {
    id: String,
    title: String,
    position: (usize, usize),
    selected: bool,
    image_path: Option<PathBuf>,
    pixel_data: Option<PixelData>,
    pixel_size: u32,
    error_message: Option<String>,
    scroll_offset: usize,
}

#[derive(Debug, Clone)]
struct PixelData {
    pixels: Vec<Vec<PixelColor>>,
    width: u32,
    height: u32,
    original_width: u32,
    original_height: u32,
}

#[derive(Debug, Clone, Copy)]
struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

impl PixelColor {
    fn to_ratatui_color(self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }

    #[allow(dead_code)] // Preserved for future ASCII art mode
    fn grayscale(&self) -> u8 {
        // Standard luminance calculation
        ((0.299 * self.r as f64) + (0.587 * self.g as f64) + (0.114 * self.b as f64)) as u8
    }

    #[allow(dead_code)] // Preserved for future ASCII art mode
    fn to_block_char(self) -> &'static str {
        let gray = self.grayscale();
        match gray {
            0..=31 => " ",
            32..=63 => "░",
            64..=95 => "▒",
            96..=127 => "▓",
            128..=159 => "█",
            160..=191 => "█",
            192..=223 => "█",
            224..=255 => "█",
        }
    }
}

impl PixelArtWidget {
    pub fn new(config: PixelArtConfig) -> Self {
        let pixel_data = if let Some(ref path) = config.image_path {
            Self::load_image_sync(path, config.pixel_size.unwrap_or(32)).ok()
        } else {
            None
        };

        Self {
            id: format!("pixelart-{}-{}", config.position.row, config.position.col),
            title: config.title,
            position: (config.position.row, config.position.col),
            selected: false,
            image_path: config.image_path,
            pixel_data,
            pixel_size: config.pixel_size.unwrap_or(32),
            error_message: None,
            scroll_offset: 0,
        }
    }

    #[allow(dead_code)] // Preserved for dynamic image loading
    pub fn set_image_path(&mut self, path: PathBuf) {
        self.image_path = Some(path.clone());
        self.error_message = None;

        match Self::load_image_sync(&path, self.pixel_size) {
            Ok(data) => {
                self.pixel_data = Some(data);
                self.scroll_offset = 0;
            }
            Err(e) => {
                self.error_message = Some(format!("Error loading image: {}", e));
                self.pixel_data = None;
            }
        }
    }

    pub fn increase_pixel_size(&mut self) {
        if self.pixel_size < 128 {
            self.pixel_size *= 2;
            self.reload_image();
        }
    }

    pub fn decrease_pixel_size(&mut self) {
        if self.pixel_size > 8 {
            self.pixel_size /= 2;
            self.reload_image();
        }
    }

    fn reload_image(&mut self) {
        if let Some(ref path) = self.image_path {
            match Self::load_image_sync(path, self.pixel_size) {
                Ok(data) => {
                    self.pixel_data = Some(data);
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Error reloading image: {}", e));
                }
            }
        }
    }

    fn load_image_sync(path: &PathBuf, target_size: u32) -> anyhow::Result<PixelData> {
        // Read and decode image
        let img_bytes = std::fs::read(path)?;
        let img = image::load_from_memory(&img_bytes)?;

        let original_width = img.width();
        let original_height = img.height();

        // Calculate aspect-preserving dimensions
        let (new_width, new_height) = if original_width > original_height {
            let ratio = original_height as f64 / original_width as f64;
            (target_size, (target_size as f64 * ratio) as u32)
        } else {
            let ratio = original_width as f64 / original_height as f64;
            ((target_size as f64 * ratio) as u32, target_size)
        };

        // Resize using nearest neighbor for pixel art effect
        let resized = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);

        // Convert to RGB
        let rgb_img = resized.to_rgb8();

        // Extract pixel data
        let mut pixels = Vec::new();
        for y in 0..new_height {
            let mut row = Vec::new();
            for x in 0..new_width {
                let pixel = rgb_img.get_pixel(x, y);
                row.push(PixelColor {
                    r: pixel[0],
                    g: pixel[1],
                    b: pixel[2],
                });
            }
            pixels.push(row);
        }

        Ok(PixelData {
            pixels,
            width: new_width,
            height: new_height,
            original_width,
            original_height,
        })
    }
}

struct PixelArtFetcher;

#[async_trait]
impl FeedFetcher for PixelArtFetcher {
    async fn fetch(&self) -> anyhow::Result<FeedData> {
        // Pixel art widget doesn't fetch data
        Ok(FeedData::Loading)
    }
}

impl FeedWidget for PixelArtWidget {
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

        if let Some(error) = &self.error_message {
            let error_text = vec![
                Line::from(""),
                Line::from(Span::styled(error, Style::default().fg(Color::Red))),
            ];
            let paragraph = Paragraph::new(error_text).alignment(Alignment::Center);
            frame.render_widget(paragraph, inner);
            return;
        }

        if let Some(data) = &self.pixel_data {
            self.render_pixel_art(frame, inner, data);
        } else {
            self.render_help(frame, inner);
        }
    }

    fn update_data(&mut self, _data: FeedData) {
        // Pixel art widget doesn't use feed data
    }

    fn create_fetcher(&self) -> Box<dyn FeedFetcher> {
        Box::new(PixelArtFetcher)
    }

    fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    fn scroll_down(&mut self) {
        if let Some(data) = &self.pixel_data {
            if self.scroll_offset < data.height as usize {
                self.scroll_offset += 1;
            }
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
        None
    }
}

impl PixelArtWidget {
    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let help_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Pixel Art Widget",
                Style::default().fg(Color::Cyan),
            )),
            Line::from(""),
            Line::from("No image loaded."),
            Line::from(""),
            Line::from("Configure image_path in config.toml:"),
            Line::from(""),
            Line::from("[[widgets]]"),
            Line::from("type = \"pixelart\""),
            Line::from("title = \"Pixel Art\""),
            Line::from("image_path = \"/path/to/image.png\""),
            Line::from("pixel_size = 32"),
            Line::from("position = { row = 0, col = 0 }"),
            Line::from(""),
            Line::from("Keybindings (when selected):"),
            Line::from("  + : Increase pixel size"),
            Line::from("  - : Decrease pixel size"),
            Line::from("  ↑↓: Scroll image"),
        ];

        let paragraph = Paragraph::new(help_lines).alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }

    fn render_pixel_art(&self, frame: &mut Frame, area: Rect, data: &PixelData) {
        let mut lines = Vec::new();

        // Add metadata header
        lines.push(Line::from(vec![
            Span::styled("Image: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!(
                    "{}x{} → {}x{}",
                    data.original_width, data.original_height, data.width, data.height
                ),
                Style::default().fg(Color::White),
            ),
            Span::raw("  "),
            Span::styled("Pixel size: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", self.pixel_size),
                Style::default().fg(Color::White),
            ),
        ]));
        lines.push(Line::from(""));

        // Calculate visible rows based on available space
        let header_height = 2; // metadata + blank line
        let max_visible_rows =
            (area.height.saturating_sub(header_height) as usize).min(data.height as usize);

        let start_row = self
            .scroll_offset
            .min(data.height.saturating_sub(max_visible_rows as u32) as usize);
        let end_row = (start_row + max_visible_rows).min(data.height as usize);

        // Render pixel rows
        for row in &data.pixels[start_row..end_row] {
            let mut spans = Vec::new();
            for pixel in row {
                // Use colored blocks for truecolor support
                spans.push(Span::styled(
                    "█",
                    Style::default().fg(pixel.to_ratatui_color()),
                ));
            }
            lines.push(Line::from(spans));
        }

        // Add scroll indicator if needed
        if data.height as usize > max_visible_rows {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("Row {}/{} (use ↑↓ to scroll)", start_row + 1, data.height),
                Style::default().fg(Color::DarkGray),
            )));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    }
}
