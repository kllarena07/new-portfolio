use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct WebSocket {}

const WEBSOCKET_FG: Color = Color::Rgb(74, 121, 33);

impl WebSocket {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("websocket", WEBSOCKET_FG, WHITE)
    }
}
