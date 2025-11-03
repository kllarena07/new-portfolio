use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct SvelteKit {}

const SVELTEKIT_BG: Color = Color::Rgb(235, 81, 40);

impl SvelteKit {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("sveltekit", WHITE, SVELTEKIT_BG)
    }
}
