use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Constraint,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Table, Wrap},
};

use crate::pages::page::Page;
use crate::pages::style::{gray_span, selected_style, white_span};

struct ExperienceItem {
    role: String,
    affiliation: String,
    time: String,
}

impl ExperienceItem {
    pub const fn ref_array(&self) -> [&String; 3] {
        [&self.role, &self.affiliation, &self.time]
    }
}

pub struct Leadership {
    state: usize,
    experiences: Vec<ExperienceItem>,
}

impl Leadership {
    pub fn new() -> Self {
        let experiences = vec![
            ExperienceItem {
                role: String::from("ceo"),
                affiliation: String::from("filipino americans in tech"),
                time: String::from("(oct 2024-present)"),
            },
            ExperienceItem {
                role: String::from("coo"),
                affiliation: String::from("wecracked"),
                time: String::from("(may 2024-jul 2024)"),
            },
        ];

        Self {
            state: 0,
            experiences,
        }
    }

    fn previous_experience(&mut self) {
        if self.state > 0 {
            self.state -= 1;
        }
    }

    fn next_experience(&mut self) {
        if self.state < self.experiences.len() - 1 {
            self.state += 1;
        }
    }

    fn get_description(&self) -> Vec<Line<'_>> {
        match self.state {
            0 => {
                vec![
                    Line::from(gray_span(
                        "building the largest network of filipino tech professionals to help make tech more accessible for filipinos. currently at 263 members",
                    )),
                    Line::from(""),
                    Line::from(gray_span("notable highlights:")),
                    Line::from(gray_span(
                        "- collaborated amazon, dreamhaven, and aapi in gaming",
                    )),
                    Line::from(gray_span(
                        "- organized a hackathon to help filipino businesses that was sponsored by vercel, warp, and sorce (yc f25)",
                    )),
                ]
            }
            1 => {
                vec![
                    Line::from(gray_span("built a 6,000+ member hackathon community")),
                    Line::from(""),
                    Line::from(gray_span("notable highlights:")),
                    Line::from(gray_span(
                        "- secured $2k in sponsorship backing from companies like koyeb and tensordock",
                    )),
                ]
            }
            _ => vec![],
        }
    }
}

impl Page for Leadership {
    fn title(&self) -> &str {
        "leadership"
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let header = ["role", "affiliation", "time"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.experiences.iter().enumerate().map(|(i, data)| {
            let item = data.ref_array();

            let style_config = match i == self.state {
                true => selected_style(),
                false => Style::new().fg(Color::Rgb(147, 147, 147)),
            };

            item.into_iter()
                .map(|content| Cell::from(content.as_str()))
                .collect::<Row>()
                .style(style_config)
                .height(1)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Fill(1),
                Constraint::Fill(2),
                Constraint::Fill(2),
            ],
        )
        .header(header)
        .block(Block::new().padding(Padding {
            left: 1,
            right: 2,
            top: 0,
            bottom: 0,
        }));

        frame.render_widget(table, area);
    }

    fn render_additional(&self, frame: &mut Frame, area: Rect) {
        let mut description = self.get_description();
        description.insert(0, Line::from(vec![white_span("desc")]));

        let paragraph = Paragraph::new(description).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }

    fn keyboard_event_handler(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('k') => {
                self.previous_experience();
            }
            KeyCode::Char('j') => {
                self.next_experience();
            }
            _ => {}
        }
    }
}
