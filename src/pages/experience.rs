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
use crate::pages::style::{gray_span, line_from_spans, selected_style, white_span};

struct ExperienceItem {
    role: String,
    affiliation: String,
    time: String,
    description: Vec<String>,
}

impl ExperienceItem {
    pub const fn ref_array(&self) -> [&String; 3] {
        [&self.role, &self.affiliation, &self.time]
    }
}

pub struct Experience {
    state: usize,
    experiences: Vec<ExperienceItem>,
}

impl Experience {
    pub fn new() -> Self {
        let experiences = vec![
            ExperienceItem {
                role: String::from("swe intern"),
                affiliation: String::from("capital one"),
                time: String::from("(jun 2026-aug 2026)"),
                description: vec![String::from("incoming summer 2026 under the tip program")],
            },
            ExperienceItem {
                role: String::from("ceo / cto"),
                affiliation: String::from("ootd"),
                time: String::from("(mar 2025-oct 2025)"),
                description: vec![
                    String::from(
                        "led a team of 4 to ship an irl dress to impress mobile app with 260+ users",
                    ),
                    String::from(""),
                    String::from("notable highlights:"),
                    String::from(
                        "- achieved a 3x boost in dau retention by analyzing user behavior patterns and implementing targeted push notifications",
                    ),
                ],
            },
            ExperienceItem {
                role: String::from("swe intern"),
                affiliation: String::from("capital one"),
                time: String::from("(jun 2025-aug 2025)"),
                description: vec![
                    String::from("worked on the capital one empath dashboard on a team of 5"),
                    String::from(""),
                    String::from("notable highlights:"),
                    String::from(
                        "- created a digital enrollment status badge to help reduce Capital One agent call times by 12%",
                    ),
                ],
            },
            ExperienceItem {
                role: String::from("mobile app dev"),
                affiliation: String::from("swe, um-dearborn"),
                time: String::from("(feb 2025-mar 2025)"),
                description: vec![
                    String::from(
                        "solo developed an event management mobile app for the society of women engineers at the university of michigan-dearborn's power conference",
                    ),
                    String::from(""),
                    String::from(
                        "features include qr code check-ins and a live agenda, message feed, and push notifications to keep attendees updated",
                    ),
                    String::from(""),
                    String::from("notable highlights:"),
                    String::from("- deployed to the ios app store as 'power um-d'"),
                    String::from("- supported 80+ attendees"),
                ],
            },
            ExperienceItem {
                role: String::from("frontend dev"),
                affiliation: String::from("gdsc, um-dearborn"),
                time: String::from("(nov 2023-dec 2023)"),
                description: vec![
                    String::from("built the michigan devfest 2023 website on a team of 8"),
                    String::from(""),
                    String::from("notable highlights:"),
                    String::from("- website drove 300+ event attendees"),
                ],
            },
            ExperienceItem {
                role: String::from("fullstack dev"),
                affiliation: String::from("adhd magazine"),
                time: String::from("(may 2023-aug 2023)"),
                description: vec![
                    String::from(
                        "designed a blog platform to showcase detroit's underground culture",
                    ),
                    String::from(""),
                    String::from("notable highlights:"),
                    String::from("- engaged an audience of 2500+ followers"),
                ],
            },
            ExperienceItem {
                role: String::from("incubatee"),
                affiliation: String::from("ai camp"),
                time: String::from("(sep 2022-nov 2022)"),
                description: vec![
                    String::from(
                        "created gpt-3 wrapper that summarized videos, audio, and text as part of the 2023 ai camp incubator program",
                    ),
                    String::from(""),
                    String::from("notable highlights:"),
                    String::from("- won $500 by placing 2nd place out of 21 other teams"),
                ],
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
        let experience_index = self.state;
        let mut final_vec: Vec<Line<'_>> = vec![];
        let experience_item = &self.experiences[experience_index];

        for desc_part in &experience_item.description {
            final_vec.push(line_from_spans(vec![gray_span(&desc_part)]));
        }

        final_vec
    }
}

impl Page for Experience {
    fn title(&self) -> &str {
        "experience"
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
                Constraint::Fill(1),
                Constraint::Fill(1),
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
