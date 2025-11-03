use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct FastAPI {}

const FASTAPI_BG: Color = Color::Rgb(5, 143, 127);

impl FastAPI {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("fastapi", WHITE, FASTAPI_BG)
    }
}
