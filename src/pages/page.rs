use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

pub trait Page {
    fn title(&self) -> &str;
    fn render(&self, frame: &mut Frame, area: Rect);
    fn render_additional(&self, _frame: &mut Frame, _area: Rect) {}
    fn keyboard_event_handler(&mut self, key_code: KeyCode) -> bool;
}
