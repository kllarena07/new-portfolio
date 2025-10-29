use crossterm::event::KeyCode;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Style, Stylize},
    symbols,
    text::Span,
    widgets::{Block, Borders, List, ListItem, Padding},
};
use std::{io, sync::mpsc, thread, time::Duration};

mod pages;
use pages::{about::About, experience::Experience, page::Page};

use crate::pages::projects::Projects;

fn main() -> io::Result<()> {
    let pages: Vec<Box<dyn Page>> = vec![
        Box::new(About::new()),
        Box::new(Experience::new()),
        Box::new(Projects::new()),
    ];

    let mut app = App {
        running: true,
        selected_page: 0,
        pages,
    };

    let mut terminal = ratatui::init();

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let tx_to_counter_events = event_tx.clone();
    thread::spawn(move || {
        run_tick_thread(tx_to_counter_events, 30);
    });

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();
    app_result
}

enum Event {
    Input(crossterm::event::KeyEvent),
    Tick(u64),
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

fn run_tick_thread(tx: mpsc::Sender<Event>, fps: u64) {
    let frame_duration = Duration::from_millis(1000 / fps);
    let mut tick: u64 = 0;
    loop {
        tx.send(Event::Tick(tick)).unwrap();
        tick = tick.wrapping_add(1);
        thread::sleep(frame_duration);
    }
}

struct App {
    running: bool,
    selected_page: usize,
    pages: Vec<Box<dyn Page>>,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while self.running {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Tick(tick) => {
                    if let Some(page) = self.pages.get_mut(self.selected_page) {
                        let _ = page.on_tick(tick);
                    }
                }
            }

            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [vertical_area] = Layout::vertical([Constraint::Percentage(35)])
            .flex(Flex::Center)
            .areas(frame.area());
        let [left_area, center_area, right_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Max(80),
            Constraint::Fill(1),
        ])
        .flex(Flex::Center)
        .areas(vertical_area);
        let menu_height: u16 = (self.pages.len() + 2) as u16;
        let [menu_area] = Layout::vertical([Constraint::Max(menu_height)]).areas(left_area);
        let [vcanvas_area] = Layout::vertical([Constraint::Max(15)]).areas(right_area);
        let [canvas_area] = Layout::horizontal([Constraint::Max(50)]).areas(vcanvas_area);

        // frame.render_widget(
        //     Block::new()
        //         .fg(Color::Red)
        //         .title("Left")
        //         .borders(Borders::ALL),
        //     left_area,
        // );
        // frame.render_widget(
        //     Block::new()
        //         .fg(Color::Green)
        //         .title("Center")
        //         .borders(Borders::ALL),
        //     center_area,
        // );
        // frame.render_widget(
        //     Block::new()
        //         .fg(Color::Blue)
        //         .title("Right")
        //         .borders(Borders::ALL),
        //     canvas_area,
        // );

        let menu_widget = self.build_menu_widget();
        frame.render_widget(menu_widget, menu_area);

        if let Some(current_page) = self.pages.get(self.selected_page) {
            current_page.render(frame, center_area);
            current_page.render_additional(frame, canvas_area);
        }
    }

    fn previous_page(&mut self) {
        if self.selected_page > 0 {
            self.selected_page -= 1;
        }
    }

    fn next_page(&mut self) {
        if self.selected_page + 1 < self.pages.len() {
            self.selected_page += 1;
        }
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => {
                self.running = false;
            }
            KeyCode::Up => self.previous_page(),
            KeyCode::Down => self.next_page(),
            _ => {
                if let Some(current_page) = self.pages.get_mut(self.selected_page) {
                    current_page.keyboard_event_handler(key_event.code);
                }
            }
        }

        Ok(())
    }

    fn build_menu_widget(&self) -> List<'_> {
        let menu_items: Vec<ListItem> = (0..self.pages.len())
            .map(move |index| {
                let title = self.pages[index].title();
                let item_content = if index == self.selected_page {
                    format!("[ {} ]", title)
                } else {
                    title.to_string()
                };

                let span = if index == self.selected_page {
                    Span::styled(item_content, Style::default().fg(Color::Rgb(255, 255, 255)))
                } else {
                    Span::styled(item_content, Style::default().fg(Color::Rgb(147, 147, 147)))
                };

                ListItem::new(span.bold().into_right_aligned_line())
            })
            .collect();

        let final_list = List::new(menu_items).block(
            Block::new()
                .borders(Borders::RIGHT)
                .border_set(symbols::border::ONE_EIGHTH_TALL)
                .border_style(Style::new().fg(Color::Rgb(147, 147, 147)))
                .padding(Padding {
                    top: 1,
                    bottom: 1,
                    right: 2,
                    left: 0,
                }),
        );

        final_list
    }
}
