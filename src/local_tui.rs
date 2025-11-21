use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crate::app::App;

pub struct LocalTuiRunner;

impl LocalTuiRunner {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self) -> Result<(), anyhow::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

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

        restore_terminal()?;
        Ok(())
    }
}

fn restore_terminal() -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
