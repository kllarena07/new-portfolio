use crate::pages::page::Page;
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    prelude::Stylize,
    style::{Color, Style},
    text::{Line, Span},
    widgets::canvas::{Canvas, Points},
    widgets::{Block, Padding, Paragraph, Wrap},
};

#[derive(Clone)]
pub struct ContactLink<'a> {
    pub display_text: &'a str,
    pub link: &'a str,
}

pub struct About<'a> {
    pub name: String,
    state: usize,
    current_link: String,
    links: Vec<ContactLink<'a>>,
}

impl<'a> Page for About<'a> {
    fn title(&self) -> &str {
        &self.name
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
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

        let paragraph = Paragraph::new(vec![
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
        .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    fn render_additional(&self, frame: &mut Frame, area: Rect) {
        let canvas_width = area.width as f64;
        let canvas_height = (area.height * 2) as f64; // HalfBlock doubles vertical resolution

        Canvas::default()
            .marker(ratatui::symbols::Marker::HalfBlock)
            .x_bounds([0.0, canvas_width])
            .y_bounds([0.0, canvas_height])
            .paint(|ctx| {
                // Stretch the 112x112 frame to fill the entire canvas area
                let frame_width = 112.0;
                let frame_height = 112.0;
                // Draw pixels from the current frame, mapping each pixel to fill the canvas
                for (y, row) in current_frame.iter().enumerate() {
                    for (x, pixel) in row.iter().enumerate() {
                        // Map frame coordinates directly to canvas coordinates
                        let canvas_x = (x as f64 / frame_width) * canvas_width;
                        let canvas_y = canvas_height - ((y as f64 / frame_height) * canvas_height);
                        ctx.draw(&Points {
                            coords: &[(canvas_x, canvas_y)],
                            color: ratatui::style::Color::Rgb(pixel[0], pixel[1], pixel[2]),
                        });
                    }
                }
            });
    }

    fn keyboard_event_handler(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Left => {
                self.previous_link();
                true
            }
            KeyCode::Right => {
                self.next_link();
                true
            }
            KeyCode::Enter => {
                let _ = open::that(&self.current_link);
                true
            }
            _ => false,
        }
    }
}

impl<'a> About<'a> {
    pub fn new() -> Self {
        let links: Vec<ContactLink> = vec![
            ContactLink {
                display_text: "twitter",
                link: "https://x.com/krayondev",
            },
            ContactLink {
                display_text: "linkedin",
                link: "https://www.linkedin.com/in/kllarena07/",
            },
            ContactLink {
                display_text: "github",
                link: "https://github.com/kllarena07",
            },
            ContactLink {
                display_text: "email",
                link: "mailto:kieran.llarena@gmail.com",
            },
        ];

        Self {
            name: String::from("about"),
            state: 0,
            current_link: String::from(links[0].link),
            links,
        }
    }

    fn previous_link(&mut self) {
        if self.state > 0 {
            self.state -= 1;
            self.change_current_link();
        }
    }

    fn next_link(&mut self) {
        if self.state + 1 < self.links.len() {
            self.state += 1;
            self.change_current_link();
        }
    }

    fn change_current_link(&mut self) {
        self.current_link = String::from(self.links[self.state].link);
    }
}
