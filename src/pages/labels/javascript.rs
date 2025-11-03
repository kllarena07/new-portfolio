use crate::pages::{labels::label::ColoredLabel, style::BLACK};
use ratatui::style::Color;

pub struct JavaScript {}

const JAVASCRIPT_BG: Color = Color::Rgb(243, 225, 86);

impl JavaScript {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("javascript", BLACK, JAVASCRIPT_BG)
    }
}
