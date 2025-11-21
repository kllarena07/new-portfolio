use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

// Color constants
pub const WHITE: Color = Color::Rgb(255, 255, 255);
pub const BLACK: Color = Color::Rgb(0, 0, 0);
pub const GRAY: Color = Color::Rgb(147, 147, 147);
pub const DIMMED_WHITE_BG: Color = Color::Rgb(215, 215, 215);
pub const DIMMED_WHITE: Color = Color::Rgb(215, 215, 215);
pub const LINK_COLOR: Color = Color::Rgb(0, 255, 251);
pub const DIMMED_LINK_COLOR: Color = Color::Rgb(0, 205, 201);

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

pub fn dimmed_selected_style() -> Style {
    Style::new().fg(BLACK).bg(DIMMED_WHITE_BG)
}

pub fn dimmed_white_style() -> Style {
    Style::default().fg(DIMMED_WHITE)
}

pub fn link_style() -> Style {
    Style::default()
        .fg(LINK_COLOR)
        .add_modifier(ratatui::style::Modifier::UNDERLINED)
}

pub fn dimmed_link_style() -> Style {
    Style::default()
        .fg(DIMMED_LINK_COLOR)
        .add_modifier(ratatui::style::Modifier::UNDERLINED)
}

// Span builders for common text patterns
pub fn gray_span(text: &str) -> Span<'_> {
    Span::styled(text, gray_style())
}

pub fn white_span(text: &str) -> Span<'_> {
    Span::styled(text, white_style())
}

pub fn white_span_owned(text: String) -> Span<'static> {
    Span::styled(text, white_style())
}

pub fn dimmed_white_span_owned(text: String) -> Span<'static> {
    Span::styled(text, dimmed_white_style())
}

pub fn link_span(text: &str) -> Span<'_> {
    Span::styled(text, link_style())
}

// Line builder from spans
pub fn line_from_spans(spans: Vec<Span<'_>>) -> Line<'_> {
    Line::from(spans)
}
