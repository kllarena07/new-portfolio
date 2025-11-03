use crate::pages::{
    labels::label::ColoredLabel,
    style::{BLACK, WHITE},
};

pub struct NextJS {}

impl NextJS {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("nextjs", BLACK, WHITE)
    }
}
