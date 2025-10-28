use crossterm::event::KeyCode;
use ratatui::{
    style::Color,
    widgets::{Block, Borders},
};

pub struct Leadership;

impl Leadership {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self) -> Block<'_> {
        Block::new()
            .borders(Borders::ALL)
            .title("Leadership")
            .border_style(Color::Green)
    }

    pub fn keyboard_event_handler(&mut self, _key_code: KeyCode) -> bool {
        // No specific keyboard handling for leadership page yet
        false
    }
}