use crossterm::event::KeyCode;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::Span,
    widgets::{Block, Borders, List, ListItem, Padding},
};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App {
        selected_page: 0,
        running: true,
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

pub struct App {
    selected_page: usize,
    running: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }

            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
            let [area] = Layout::horizontal([horizontal])
                .flex(Flex::Center)
                .areas(area);
            let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
            area
        }

        let portfolio_area = center(
            frame.area(),
            Constraint::Length(150),
            Constraint::Length(25),
        );

        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                // must add up to 100
                Constraint::Max(17),
                Constraint::Percentage(57),
                Constraint::Min(26),
            ])
            .split(portfolio_area);

        let menu = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(6)])
            .split(outer_layout[0]);

        let unselected_items: [ListItem; 4] = [
            ListItem::new(
                Span::styled("about", Style::default().fg(Color::DarkGray))
                    .into_right_aligned_line(),
            ),
            ListItem::new(
                Span::styled("experience", Style::default().fg(Color::DarkGray))
                    .into_right_aligned_line(),
            ),
            ListItem::new(
                Span::styled("projects", Style::default().fg(Color::DarkGray))
                    .into_right_aligned_line(),
            ),
            ListItem::new(
                Span::styled("leadership", Style::default().fg(Color::DarkGray))
                    .into_right_aligned_line(),
            ),
        ];
        let selected_items: [ListItem; 4] = [
            ListItem::new(
                Span::styled("[ about ]", Style::default().fg(Color::White).bold())
                    .into_right_aligned_line(),
            ),
            ListItem::new(
                Span::styled("[ experience ]", Style::default().fg(Color::White).bold())
                    .into_right_aligned_line(),
            ),
            ListItem::new(
                Span::styled("[ projects ]", Style::default().fg(Color::White).bold())
                    .into_right_aligned_line(),
            ),
            ListItem::new(
                Span::styled("[ leadership ]", Style::default().fg(Color::White).bold())
                    .into_right_aligned_line(),
            ),
        ];
        // items[0].width()

        let menu_items: Vec<ListItem> = unselected_items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                if index == self.selected_page {
                    selected_items[index].clone()
                } else {
                    item.clone()
                }
            })
            .collect();

        frame.render_widget(
            List::new(menu_items).block(
                Block::new()
                    .borders(Borders::RIGHT)
                    .border_set(symbols::border::ONE_EIGHTH_TALL)
                    .border_style(Style::new().fg(Color::DarkGray))
                    .padding(Padding {
                        top: 1,
                        bottom: 0,
                        right: 2,
                        left: 0,
                    }),
            ),
            menu[0],
        );
        frame.render_widget(
            Block::new().bold().fg(Color::Green).borders(Borders::ALL),
            // Block::new(),
            outer_layout[1],
        );
        frame.render_widget(
            Block::new().bold().fg(Color::Blue).borders(Borders::ALL),
            // Block::new(),
            outer_layout[2],
        );
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => {
                self.running = false;
            }
            KeyCode::Up => {
                if self.selected_page > 0 {
                    self.selected_page -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_page < 3 {
                    self.selected_page += 1;
                }
            }
            _ => {}
        }

        Ok(())
    }
}
