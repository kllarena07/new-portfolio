use crate::pages::{
    labels::label::ColoredLabel,
    style::{BLACK, WHITE},
};

pub struct KindeAuth {}

impl KindeAuth {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("kinde auth", BLACK, WHITE)
    }
}
