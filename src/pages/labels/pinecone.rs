use crate::pages::{
    labels::label::ColoredLabel,
    style::{BLACK, WHITE},
};

pub struct Pinecone {}

impl Pinecone {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("pinecone", BLACK, WHITE)
    }
}
