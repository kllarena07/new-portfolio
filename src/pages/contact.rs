use crate::pages::page::Page;
use crate::pages::style::{gray_span, white_span};
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Padding, Paragraph, Wrap},
};

#[derive(Clone)]
pub struct ContactLink<'a> {
    pub display_text: &'a str,
    pub link: &'a str,
}

pub struct Contact<'a> {
    links: Vec<ContactLink<'a>>,
}

impl<'a> Page for Contact<'a> {
    fn title(&self) -> &str {
        "contact"
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let links: Vec<Line<'_>> = (0..(self.links.len()))
            .map(|index| {
                let current_contact_link = &self.links[index];
                Line::from(vec![
                    white_span(current_contact_link.display_text),
                    white_span(": "),
                    gray_span(current_contact_link.link),
                ])
            })
            .collect();

        let paragraph = Paragraph::new(links)
            .block(Block::new().padding(Padding {
                left: 1,
                right: 2,
                top: 0,
                bottom: 0,
            }))
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    fn render_additional(&self, _frame: &mut Frame, _area: Rect) {}

    fn keyboard_event_handler(&mut self, _key_code: KeyCode) {}
}

impl<'a> Contact<'a> {
    pub fn new() -> Self {
        let links: Vec<ContactLink> = vec![
            ContactLink {
                display_text: "twitter",
                link: "x.com/krayondev",
            },
            ContactLink {
                display_text: "linkedin",
                link: "linkedin.com/in/kllarena07/",
            },
            ContactLink {
                display_text: "github",
                link: "github.com/kllarena07",
            },
            ContactLink {
                display_text: "email",
                link: "kieran.llarena@gmail.com",
            },
        ];

        Self { links }
    }
}
