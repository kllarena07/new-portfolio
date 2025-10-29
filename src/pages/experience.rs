use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Constraint,
    layout::Rect,
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Table, Wrap},
};

use crate::pages::page::Page;
use crate::pages::style::{gray_span, gray_style, line_from_spans, selected_style, white_span};

struct ExperienceItem {
    role: &'static str,
    affiliation: &'static str,
    time: &'static str,
    description: Vec<&'static str>,
}

impl ExperienceItem {
    pub const fn ref_array(&self) -> [&str; 3] {
        [self.role, self.affiliation, self.time]
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
                role: "swe intern",
                affiliation: "capital one",
                time: "(jun 2026-aug 2026)",
                description: vec!["incoming summer 2026 under the tip program"],
            },
            ExperienceItem {
                role: "ceo / cto",
                affiliation: "ootd",
                time: "(mar 2025-oct 2025)",
                description: vec![
                    "led a team of 4 to ship an irl dress to impress mobile app with 260+ users",
                    "",
                    "notable highlights:",
                    "- achieved a 3x boost in dau retention by analyzing user behavior patterns and implementing targeted push notifications",
                ],
            },
            ExperienceItem {
                role: "swe intern",
                affiliation: "capital one",
                time: "(jun 2025-aug 2025)",
                description: vec![
                    "worked on the capital one empath dashboard on a team of 5",
                    "",
                    "notable highlights:",
                    "- created a digital enrollment status badge to help reduce capital one agent call times by 12%",
                ],
            },
            ExperienceItem {
                role: "mobile app dev",
                affiliation: "swe, um-dearborn",
                time: "(feb 2025-mar 2025)",
                description: vec![
                    "solo developed an event management mobile app for the society of women engineers at the university of michigan-dearborn's power conference",
                    "",
                    "features include qr code check-ins and a live agenda, message feed, and push notifications to keep attendees updated",
                    "",
                    "notable highlights:",
                    "- deployed to the ios app store as 'power um-d'",
                    "- supported 80+ attendees",
                ],
            },
            ExperienceItem {
                role: "frontend dev",
                affiliation: "gdsc, um-dearborn",
                time: "(nov 2023-dec 2023)",
                description: vec![
                    "built the michigan devfest 2023 website on a team of 8",
                    "",
                    "notable highlights:",
                    "- website drove 300+ event attendees",
                ],
            },
            ExperienceItem {
                role: "fullstack dev",
                affiliation: "adhd magazine",
                time: "(may 2023-aug 2023)",
                description: vec![
                    "designed a blog platform to showcase detroit's underground culture",
                    "",
                    "notable highlights:",
                    "- engaged an audience of 2500+ followers",
                ],
            },
            ExperienceItem {
                role: "incubatee",
                affiliation: "ai camp",
                time: "(sep 2022-nov 2022)",
                description: vec![
                    "created gpt-3 wrapper that summarized videos, audio, and text as part of the 2023 ai camp incubator program",
                    "",
                    "notable highlights:",
                    "- won $500 by placing 2nd place out of 21 other teams",
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
                false => gray_style(),
            };

            item.into_iter()
                .map(|content| Cell::from(content))
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
        description.insert(0, line_from_spans(vec![white_span("desc")]));

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
