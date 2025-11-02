use crate::pages::labels::label::ColoredLabel;
use ratatui::style::Color;

pub struct Supabase {}

const SUPABASE_FG: Color = Color::Rgb(38, 204, 137);
const SUPABASE_BG: Color = Color::Rgb(33, 33, 33);

impl Supabase {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("supabase", SUPABASE_FG, SUPABASE_BG)
    }
}
