use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct React {}

const REACT_BG: Color = Color::Rgb(56, 124, 161);

impl React {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("react", WHITE, REACT_BG)
    }
}
