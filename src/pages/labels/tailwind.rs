use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct Tailwind {}

const TAILWIND_FG: Color = Color::Rgb(83, 182, 213);

impl Tailwind {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("tailwind", TAILWIND_FG, WHITE)
    }
}
