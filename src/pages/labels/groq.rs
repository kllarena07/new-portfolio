use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct Groq {}

const GROQ_BG: Color = Color::Rgb(251, 62, 49);

impl Groq {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("groq", WHITE, GROQ_BG)
    }
}
