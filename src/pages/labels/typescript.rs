use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct TypeScript {}

const TYPESCRIPT_BG: Color = Color::Rgb(69, 119, 194);

impl TypeScript {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("typescript", WHITE, TYPESCRIPT_BG)
    }
}
