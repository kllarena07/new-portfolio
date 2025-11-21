use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct Modal {}

const MODAL_BG: Color = Color::Rgb(106, 239, 91);

impl Modal {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("modal", WHITE, MODAL_BG)
    }
}
