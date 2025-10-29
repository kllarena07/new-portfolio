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
    description: Vec<String>,
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
                description: vec![
                    String::from(
                        "building the largest network of filipino tech professionals to help make tech more accessible for filipinos. currently at 263 members",
                    ),
                    String::from(""),
                    String::from("notable highlights:"),
                    String::from("- collaborated amazon, dreamhaven, and aapi in gaming"),
                    String::from(
                        "- organized a hackathon to help filipino businesses that was sponsored by vercel, warp, and sorce (yc f25)",
                    ),
                ],
            },
            ExperienceItem {
                role: String::from("coo"),
                affiliation: String::from("wecracked"),
                time: String::from("(may 2024-jul 2024)"),
                description: vec![
                    String::from("built a 6,000+ member hackathon community"),
                    String::from(""),
                    String::from("notable highlights:"),
                    String::from(
                        "- secured $2k in sponsorship backing from companies like koyeb and tensordock",
                    ),
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
            final_vec.push(Line::from(vec![gray_span(&desc_part)]));
        }

        final_vec
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
