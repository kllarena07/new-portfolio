use crossterm::event::KeyCode;
use ratatui::{
    prelude::Stylize,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Wrap},
};

#[derive(Clone)]
pub struct ContactLink {
    pub display_text: String,
    pub link: String,
}

pub struct About {
    pub state: usize,
    pub current_link: String,
    pub links: Vec<ContactLink>,
}

impl About {
    pub fn new(links: Vec<ContactLink>) -> Self {
        let current_link = links[0].link.clone();
        Self {
            state: 0,
            current_link,
            links,
        }
    }

    pub fn build(&self) -> Paragraph<'_> {
        let line_1 = Line::from(vec![
            Span::styled(
                "hey! my name is ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "kieran llarena",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_2 = Line::from(vec![
            Span::styled(
                "im currently studying ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "computer science ",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
            Span::styled("at the ", Style::default().fg(Color::Rgb(147, 147, 147))),
            Span::styled(
                "university of michigan-dearborn",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_3 = Line::from(vec![
            Span::styled(
                "my expected graduation date is ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled("may 2027", Style::default().fg(Color::Rgb(255, 255, 255))),
        ]);

        let line_4 = Line::from(vec![
            Span::styled(
                "i thrive best in environments that value ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "high velocity ",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
            Span::styled("and ", Style::default().fg(Color::Rgb(147, 147, 147))),
            Span::styled(
                "strong ownership",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_5 = Line::from(vec![
            Span::styled(
                "my background is rooted in ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "web and mobile fullstack development",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_6 = Line::from(vec![
            Span::styled(
                "im currently exploring ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "systems programming",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
            Span::styled(
                ", specifically working with ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "embedded Rust on microcontrollers",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_items: Vec<Span> = (0..(self.links.len() * 2) - 1)
            .map(move |index| {
                if (index + 1) % 2 == 0 {
                    return Span::styled(" - ", Style::default().fg(Color::Rgb(147, 147, 147)));
                }

                let style_config = match index / 2 == self.state {
                    true => Style::default().fg(Color::Rgb(0, 255, 251)).underlined(),
                    false => Style::default().fg(Color::Rgb(147, 147, 147)),
                };

                let display_text = self.links[index / 2].display_text.to_owned();

                Span::styled(display_text, style_config)
            })
            .collect();

        let links_line = Line::from(line_items);

        Paragraph::new(vec![
            line_1,
            Line::from(""),
            line_2,
            Line::from(""),
            line_3,
            Line::from(""),
            line_4,
            Line::from(""),
            line_5,
            Line::from(""),
            line_6,
            Line::from(""),
            links_line,
        ])
        .block(Block::new().padding(Padding {
            left: 1,
            right: 2,
            top: 0,
            bottom: 0,
        }))
        .wrap(Wrap { trim: true })
    }

    pub fn keyboard_event_handler(&mut self, key_code: KeyCode) -> Option<String> {
        match key_code {
            KeyCode::Left => {
                self.previous_link();
                None
            }
            KeyCode::Right => {
                self.next_link();
                None
            }
            KeyCode::Enter => {
                if !self.current_link.is_empty() {
                    Some(self.current_link.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn previous_link(&mut self) {
        if self.state > 0 {
            self.state -= 1;
            self.current_link = self.links[self.state].link.to_owned();
        }
    }

    fn next_link(&mut self) {
        if self.state < self.links.len() - 1 {
            self.state += 1;
            self.current_link = self.links[self.state].link.to_owned();
        }
    }
}