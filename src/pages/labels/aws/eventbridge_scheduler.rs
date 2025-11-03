use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct EventBridgeScheduler {}

const EVENTBRIDGE_BG: Color = Color::Rgb(207, 52, 118);

impl EventBridgeScheduler {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("eventbridge scheduler", WHITE, EVENTBRIDGE_BG)
    }
}
