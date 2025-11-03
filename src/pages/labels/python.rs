use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct Python {}

const PYTHON_BG: Color = Color::Rgb(5, 143, 127);

impl Python {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("python", WHITE, PYTHON_BG)
    }
}
