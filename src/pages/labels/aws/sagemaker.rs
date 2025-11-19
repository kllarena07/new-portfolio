use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct SageMaker {}

const SAGEMAKER_BG: Color = Color::Rgb(102, 44, 221);

impl SageMaker {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("sagemaker", WHITE, SAGEMAKER_BG)
    }
}
