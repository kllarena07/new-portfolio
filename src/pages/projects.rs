use crossterm::event::KeyCode;
use ratatui::{
    style::Color,
    widgets::{Block, Borders},
};

pub struct Projects;

impl Projects {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self) -> Block<'_> {
        Block::new()
            .borders(Borders::ALL)
            .title("Projects")
            .border_style(Color::Red)
    }

    pub fn keyboard_event_handler(&mut self, _key_code: KeyCode) -> bool {
        // No specific keyboard handling for projects page yet
        false
    }
}