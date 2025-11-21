use crossterm::event::{self, Event};

use crate::app::App;

pub struct LocalTuiRunner;

impl LocalTuiRunner {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self) -> Result<(), anyhow::Error> {
        let mut terminal = ratatui::init();

        let mut app = App::new();
        let mut tick: u64 = 0;

        loop {
            terminal.draw(|f| {
                app.draw(f);
            })?;

            if event::poll(tokio::time::Duration::from_millis(1000 / 30))? {
                if let Event::Key(key) = event::read()? {
                    match app.handle_key_event(key.code) {
                        Ok(_) => {}
                        Err(_) => break,
                    }
                }
            }

            app.handle_tick(tick);
            tick = tick.wrapping_add(1);
        }

        ratatui::restore();
        Ok(())
    }
}
