use crate::pages::{
    labels::label::ColoredLabel,
    style::{BLACK, WHITE},
};

pub struct Flask {}

impl Flask {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("flask", WHITE, BLACK)
    }
}
