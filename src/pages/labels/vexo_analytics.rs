use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct VexoAnalytics {}

const VEXO_ANALYTICS_BG: Color = Color::Rgb(98, 83, 188);

impl VexoAnalytics {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("vexo analytics", WHITE, VEXO_ANALYTICS_BG)
    }
}
