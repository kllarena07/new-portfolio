use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct ReactNative {}

const REACT_NATIVE_BG: Color = Color::Rgb(56, 124, 161);

impl ReactNative {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("react native", WHITE, REACT_NATIVE_BG)
    }
}
