use crate::pages::{
    labels::label::ColoredLabel,
    style::{BLACK, WHITE},
};

pub struct RetellAI {}

impl RetellAI {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("retell ai", BLACK, WHITE)
    }
}
