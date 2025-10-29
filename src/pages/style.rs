use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::text::Span;

// Color constants
pub const WHITE: Color = Color::Rgb(255, 255, 255);
pub const BLACK: Color = Color::Rgb(0, 0, 0);
pub const GRAY: Color = Color::Rgb(147, 147, 147);
pub const ACCENT: Color = Color::Rgb(0, 255, 251);

// Style presets
pub fn gray_style() -> Style {
    Style::default().fg(GRAY)
}

pub fn white_style() -> Style {
    Style::default().fg(WHITE)
}

pub fn black_style() -> Style {
    Style::default().fg(BLACK)
}

pub fn accent_style() -> Style {
    Style::default().fg(ACCENT)
}

pub fn accent_underlined_style() -> Style {
    Style::default().fg(ACCENT).underlined()
}

pub fn selected_style() -> Style {
    Style::new().fg(BLACK).bg(WHITE)
}

// Span builders for common text patterns
pub fn gray_span(text: &str) -> Span {
    Span::styled(text, gray_style())
}

pub fn white_span(text: &str) -> Span {
    Span::styled(text, white_style())
}

pub fn accent_span(text: &str) -> Span {
    Span::styled(text, accent_style())
}

pub fn accent_underlined_span(text: &str) -> Span {
    Span::styled(text, accent_underlined_style())
}
