use crate::pages::{
    labels::label::ColoredLabel,
    style::{BLACK, WHITE},
};

pub struct Expo {}

impl Expo {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("expo", BLACK, WHITE)
    }
}
