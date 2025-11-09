use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

// Color constants
pub const WHITE: Color = Color::Rgb(255, 255, 255);
pub const BLACK: Color = Color::Rgb(0, 0, 0);
pub const GRAY: Color = Color::Rgb(147, 147, 147);

// Style presets
pub fn gray_style() -> Style {
    Style::default().fg(GRAY)
}

pub fn white_style() -> Style {
    Style::default().fg(WHITE)
}

pub fn selected_style() -> Style {
    Style::new().fg(BLACK).bg(WHITE)
}

// Span builders for common text patterns
pub fn gray_span(text: &str) -> Span<'_> {
    Span::styled(text, gray_style())
}

pub fn white_span(text: &str) -> Span<'_> {
    Span::styled(text, white_style())
}

// Line builder from spans
pub fn line_from_spans(spans: Vec<Span<'_>>) -> Line<'_> {
    Line::from(spans)
}
