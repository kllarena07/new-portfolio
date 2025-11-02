use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct CloudflareImages {}

const CLOUDFLARE_IMAGES_BG: Color = Color::Rgb(255, 128, 0);

impl CloudflareImages {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("cloudflare images", WHITE, CLOUDFLARE_IMAGES_BG)
    }
}
