use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout},
    style::{Color, Style, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
};
use std::io;

use crate::pages::{
    page::Page,
    style::{GRAY, dimmed_white_span_owned, gray_span, white_span, white_span_owned},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusMode {
    PageFocus,
    ContentFocus,
}

pub struct App {
    pub running: bool,
    pub selected_page: usize,
    pub pages: Vec<Box<dyn Page>>,
    pub show_left: bool,
    pub show_center: bool,
    pub show_right: bool,
    pub show_menu: bool,
    pub show_aa1: bool,
    pub show_additional: bool,
    pub focus_mode: FocusMode,
}

impl App {
    pub fn new() -> Self {
        let show_widgets = std::env::var("SHOW_WIDGETS").unwrap_or_default();
        let show_left = show_widgets == "LEFT" || show_widgets == "ALL";
        let show_center = show_widgets == "CENTER" || show_widgets == "ALL";
        let show_right = show_widgets == "RIGHT" || show_widgets == "ALL";
        let show_menu = show_widgets == "MENU" || show_widgets == "ALL";
        let show_aa1 = show_widgets == "AA1" || show_widgets == "ALL";
        let show_additional = show_widgets == "ADDITIONAL" || show_widgets == "ALL";
        let debug_frames = std::env::var("FRAME_DEBUG").unwrap_or_default();
        let show_debug_frames = debug_frames == "TRUE" || debug_frames == "true";

        let pages: Vec<Box<dyn Page>> = vec![
            Box::new(crate::pages::about::About::new(show_debug_frames)),
            Box::new(crate::pages::experience::Experience::new()),
            Box::new(crate::pages::projects::Projects::new()),
            Box::new(crate::pages::leadership::Leadership::new()),
        ];

        Self {
            running: true,
            selected_page: 0,
            pages,
            show_left,
            show_center,
            show_right,
            show_menu,
            show_aa1,
            show_additional,
            focus_mode: FocusMode::PageFocus,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let terminal_width = frame.area().width;
        if terminal_width < 150 {
            let centered_area = Layout::vertical([
                Constraint::Percentage(50),
                Constraint::Length(2),
                Constraint::Percentage(50),
            ])
            .flex(Flex::Center)
            .split(frame.area())[1];
            frame.render_widget(
                Paragraph::new("Terminal too narrow\nMinimum width: 150 columns")
                    .style(Style::new().fg(Color::Red))
                    .alignment(Alignment::Center),
                centered_area,
            );
            return;
        }

        let [vertical_area] = Layout::vertical([Constraint::Percentage(50)])
            .flex(Flex::Center)
            .areas(frame.area());

        let max_menu_width = self
            .pages
            .iter()
            .map(|page| format!("[ {} ]", page.title()).len())
            .max()
            .unwrap_or(0) as u16
            + 3; // +3 for right padding. Magic number

        let [left_area, center_area, right_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Max(80),
            Constraint::Min(50),
        ])
        .flex(Flex::Center)
        .areas(vertical_area);

        let menu_height: u16 = (self.pages.len() + 2) as u16;
        let [menu_area, below_menu_full_area] =
            Layout::vertical([Constraint::Max(menu_height), Constraint::Min(0)]).areas(left_area);

        let [_, below_menu_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Max(max_menu_width)])
                .areas(below_menu_full_area);

        let [vcanvas_area] = Layout::vertical([Constraint::Max(15)]).areas(right_area);
        let [canvas_area] = Layout::horizontal([Constraint::Max(50)]).areas(vcanvas_area);
        let [additional_area] = Layout::horizontal([Constraint::Max(50)]).areas(right_area);

        if self.show_left {
            frame.render_widget(
                Block::new()
                    .fg(Color::Red)
                    .title("Left")
                    .borders(Borders::ALL),
                left_area,
            );
        }
        if self.show_menu {
            frame.render_widget(
                Block::new()
                    .fg(Color::Green)
                    .title("Menu")
                    .borders(Borders::ALL),
                menu_area,
            );
        }
        if self.show_center {
            frame.render_widget(
                Block::new()
                    .fg(Color::Green)
                    .title("Center")
                    .borders(Borders::ALL),
                center_area,
            );
        }
        if self.show_right || self.show_aa1 {
            frame.render_widget(
                Block::new()
                    .fg(Color::Blue)
                    .title("Right")
                    .borders(Borders::ALL),
                canvas_area,
            );
        }
        if self.show_right || self.show_additional {
            frame.render_widget(
                Block::new()
                    .fg(Color::Blue)
                    .title("Right")
                    .borders(Borders::ALL),
                additional_area,
            );
        }

        let menu_widget = self.build_menu_widget();
        frame.render_widget(menu_widget, menu_area);

        let nav_widget = self.build_nav_widget();
        frame.render_widget(nav_widget, below_menu_area);

        let content_focused = self.focus_mode == FocusMode::ContentFocus;
        if let Some(current_page) = self.pages.get(self.selected_page) {
            current_page.render(frame, center_area, content_focused);
            match self.selected_page == 0 {
                true => current_page.render_additional(frame, canvas_area, content_focused),
                false => current_page.render_additional(frame, additional_area, content_focused),
            }
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyCode) -> io::Result<()> {
        match key_event {
            KeyCode::Char('q') => {
                self.running = false;
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "Quit requested",
                ));
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.focus_mode = FocusMode::PageFocus;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.focus_mode = FocusMode::ContentFocus;
            }
            KeyCode::Up | KeyCode::Char('k') => match self.focus_mode {
                FocusMode::PageFocus => self.previous_page(),
                FocusMode::ContentFocus => {
                    if let Some(current_page) = self.pages.get_mut(self.selected_page) {
                        current_page.keyboard_event_handler(KeyCode::Up);
                    }
                }
            },
            KeyCode::Down | KeyCode::Char('j') => match self.focus_mode {
                FocusMode::PageFocus => self.next_page(),
                FocusMode::ContentFocus => {
                    if let Some(current_page) = self.pages.get_mut(self.selected_page) {
                        current_page.keyboard_event_handler(KeyCode::Down);
                    }
                }
            },
            _ => {
                if self.focus_mode == FocusMode::ContentFocus {
                    if let Some(current_page) = self.pages.get_mut(self.selected_page) {
                        current_page.keyboard_event_handler(key_event);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn handle_tick(&mut self, tick: u64) {
        if let Some(page) = self.pages.get_mut(self.selected_page) {
            let _ = page.on_tick(tick);
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

    fn build_menu_widget(&self) -> List<'_> {
        let menu_items: Vec<ListItem> = (0..self.pages.len())
            .map(move |index| {
                let title = self.pages[index].title();
                let is_selected = index == self.selected_page;
                let page_focused = self.focus_mode == FocusMode::PageFocus;

                let span = if is_selected {
                    if page_focused {
                        white_span_owned(format!("[ {} ]", title))
                    } else {
                        dimmed_white_span_owned(format!("[ {} ]", title))
                    }
                } else {
                    gray_span(&title)
                };

                ListItem::new(span.bold().into_right_aligned_line())
            })
            .collect();

        let final_list = List::new(menu_items).block(
            Block::new()
                .borders(Borders::RIGHT)
                .border_set(symbols::border::ONE_EIGHTH_TALL)
                .border_style(Style::new().fg(GRAY))
                .padding(Padding {
                    top: 1,
                    bottom: 1,
                    right: 2,
                    left: 0,
                }),
        );

        final_list
    }

    fn build_nav_widget(&self) -> List<'_> {
        let focus_text = match self.focus_mode {
            FocusMode::PageFocus => "page",
            FocusMode::ContentFocus => "content",
        };

        let mut nav_lines: Vec<ListItem> = vec![
            ListItem::new(Line::from(vec![white_span("↑/↓ "), gray_span(focus_text)])),
            ListItem::new(Line::from(vec![white_span("←/→ "), gray_span("focus")])),
        ];

        if let Some(current_page) = self.pages.get(self.selected_page) {
            let page_nav_items = current_page.nav_items();
            nav_lines.extend(page_nav_items.into_iter().map(ListItem::new));
        }

        let quit_nav_item = ListItem::new(Line::from(vec![white_span(" q  "), gray_span("quit")]));

        nav_lines.push(quit_nav_item);

        let final_list = List::new(nav_lines).block(Block::new().padding(Padding {
            left: 4,
            right: 0,
            top: 0,
            bottom: 0,
        }));

        final_list
    }
}
