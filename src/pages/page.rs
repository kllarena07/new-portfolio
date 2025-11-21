use crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect, text::Line};

pub trait Page: Send + Sync {
    fn title(&self) -> &str;
    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool);
    fn render_additional(&self, frame: &mut Frame, area: Rect, is_focused: bool);
    fn keyboard_event_handler(&mut self, key_code: KeyCode);
    fn on_tick(&mut self, _tick: u64) -> bool {
        false
    }
    fn nav_items(&self) -> Vec<Line<'static>> {
        vec![]
    }
}
