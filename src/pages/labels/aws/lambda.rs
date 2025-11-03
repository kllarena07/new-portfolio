use crate::pages::{labels::label::ColoredLabel, style::BLACK};
use ratatui::style::Color;

pub struct Lambda {}

const LAMBDA_BG: Color = Color::Rgb(255, 153, 0);

impl Lambda {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("lambda", BLACK, LAMBDA_BG)
    }
}
