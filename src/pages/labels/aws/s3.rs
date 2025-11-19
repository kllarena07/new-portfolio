use crate::pages::{labels::label::ColoredLabel, style::BLACK};
use ratatui::style::Color;

pub struct S3 {}

const S3_BG: Color = Color::Rgb(209, 93, 75);

impl S3 {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("s3", BLACK, S3_BG)
    }
}
