use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table, Wrap},
};
use std::env;

use crate::pages::{
    labels::{
        cloudflare::{images::CloudflareImages, workers::CloudflareWorkers},
        javascript::JavaScript,
        label::ColoredLabel,
        pocketbase::PocketBase,
        react::react::React,
        supabase::Supabase,
        sveltekit::SvelteKit,
        tailwind::Tailwind,
        typescript::TypeScript,
        vexo_analytics::VexoAnalytics,
    },
    style::{
        WHITE, dimmed_selected_style, gray_span, gray_style, line_from_spans, selected_style,
        white_span,
    },
};
use crate::pages::{
    labels::{container::LabelContainer, expo::Expo, react::react_native::ReactNative},
    page::Page,
};

struct ExperienceItem {
    role: &'static str,
    affiliation: &'static str,
    time: &'static str,
    description: Vec<&'static str>,
    technologies: Vec<ColoredLabel>,
}

impl ExperienceItem {
    pub const fn ref_array(&self) -> [&str; 3] {
        [self.role, self.affiliation, self.time]
    }
}

pub struct Experience {
    state: usize,
    experiences: Vec<ExperienceItem>,
    show_tech_block: bool,
}

impl Experience {
    pub fn new() -> Self {
        let show_widgets = env::var("SHOW_WIDGETS").unwrap_or_default();
        let show_tech_block = show_widgets == "TECH" || show_widgets == "ALL";

        let experiences = vec![
            ExperienceItem {
                role: "swe intern",
                affiliation: "capital one",
                time: "(jun 2026-aug 2026)",
                description: vec!["incoming summer 2026 under the tip program"],
                technologies: vec![],
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
                technologies: vec![
                    Expo::build(),
                    Supabase::build(),
                    ReactNative::build(),
                    CloudflareImages::build(),
                    CloudflareWorkers::build(),
                    VexoAnalytics::build(),
                    TypeScript::build(),
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
                technologies: vec![JavaScript::build()],
            },
            ExperienceItem {
                role: "mobile app dev",
                affiliation: "swe @ um-dearborn",
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
                technologies: vec![Expo::build(), TypeScript::build(), Supabase::build()],
            },
            ExperienceItem {
                role: "mobile app dev",
                affiliation: "hackdearborn 3",
                time: "(jun 2024-oct 2024)",
                description: vec![
                    "worked on the event management mobile app for hackdearborn 3; collaborated on a team of 16",
                    "",
                    "features include qr code check-ins and a live agenda, message feed, and push notifications to keep attendees updated",
                    "",
                    "notable highlights:",
                    "- deployed to the ios app store as 'hack dearborn'",
                    "- supported 350+ participants",
                ],
                technologies: vec![Expo::build(), TypeScript::build(), Supabase::build()],
            },
            ExperienceItem {
                role: "frontend dev",
                affiliation: "gdsc @ um-dearborn",
                time: "(nov 2023-dec 2023)",
                description: vec![
                    "built the michigan devfest 2023 website on a team of 8",
                    "",
                    "notable highlights:",
                    "- website drove 300+ event attendees",
                ],
                technologies: vec![JavaScript::build(), React::build(), Tailwind::build()],
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
                technologies: vec![SvelteKit::build(), PocketBase::build(), JavaScript::build()],
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
                technologies: vec![JavaScript::build(), React::build(), Tailwind::build()],
            },
        ];

        Self {
            state: 0,
            experiences,
            show_tech_block,
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

    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let header = ["role", "affiliation", "time"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.experiences.iter().enumerate().map(|(i, data)| {
            let item = data.ref_array();

            let style_config = match i == self.state {
                true => {
                    if is_focused {
                        selected_style()
                    } else {
                        dimmed_selected_style()
                    }
                }
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
                Constraint::Length(19),
            ],
        )
        .header(header)
        .block(Block::new().padding(Padding {
            left: 1,
            right: 2,
            top: 1,
            bottom: 0,
        }));

        frame.render_widget(table, area);
    }

    fn render_additional(&self, frame: &mut Frame, area: Rect, _is_focused: bool) {
        let mut description = self.get_description();
        description.insert(0, line_from_spans(vec![white_span("desc")]));

        let paragraph = Paragraph::new(description).wrap(Wrap { trim: true });

        let text_height = paragraph.line_count(area.width); // NOTE: this feature is experimental and potentially unstable
        let available_height = area.height;

        // Ensure we don't exceed available height and reserve space for tech block
        let actual_text_height = (text_height as u16).min(available_height.saturating_sub(4));

        let [text_area, tech_area] =
            Layout::vertical([Constraint::Length(actual_text_height), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        frame.render_widget(paragraph, text_area);

        if self.show_tech_block {
            let tech_block = Block::new()
                .title("tech")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(WHITE))
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                });

            frame.render_widget(tech_block, tech_area);
        }

        let experience_item = &self.experiences[self.state];
        let container = LabelContainer::new(&experience_item.technologies);
        container.render(frame, tech_area);
    }

    fn keyboard_event_handler(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('k') | KeyCode::Up => {
                self.previous_experience();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.next_experience();
            }
            _ => {}
        }
    }
}
