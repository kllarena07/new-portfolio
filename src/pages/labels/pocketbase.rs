use crate::pages::{
    labels::label::ColoredLabel,
    style::{BLACK, WHITE},
};

pub struct PocketBase {}

impl PocketBase {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("pocketbase", BLACK, WHITE)
    }
}
