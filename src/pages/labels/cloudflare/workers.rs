use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct CloudflareWorkers {}

const CLOUDFLARE_IMAGES_BG: Color = Color::Rgb(255, 128, 0);

impl CloudflareWorkers {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("cloudflare workers", WHITE, CLOUDFLARE_IMAGES_BG)
    }
}
