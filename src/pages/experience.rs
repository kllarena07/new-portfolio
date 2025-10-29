use crossterm::event::KeyCode;
use ratatui::{
    layout::Constraint,
    prelude::Stylize,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Cell, Padding, Paragraph, Row, Table, Wrap},
};

pub struct ExperienceItem {
    pub role: String,
    pub affiliation: String,
    pub time: String,
}

impl ExperienceItem {
    pub const fn ref_array(&self) -> [&String; 3] {
        [&self.role, &self.affiliation, &self.time]
    }
}

pub struct Experience {
    pub state: usize,
    pub experiences: Vec<ExperienceItem>,
}

impl Experience {
    pub fn new(experiences: Vec<ExperienceItem>) -> Self {
        let _experience_items = vec![
            ExperienceItem {
                role: String::from("swe intern"),
                affiliation: String::from("capital one"),
                time: String::from("(jun 2026-aug 2026)"),
            },
            ExperienceItem {
                role: String::from("ceo / cto"),
                affiliation: String::from("ootd"),
                time: String::from("(mar 2025-oct 2025)"),
            },
            ExperienceItem {
                role: String::from("swe intern"),
                affiliation: String::from("capital one"),
                time: String::from("(jun 2025-aug 2025)"),
            },
            ExperienceItem {
                role: String::from("mobile app dev"),
                affiliation: String::from("swe, um-dearborn"),
                time: String::from("(feb 2025-mar 2025)"),
            },
            ExperienceItem {
                role: String::from("frontend dev"),
                affiliation: String::from("gdsc, um-dearborn"),
                time: String::from("(nov 2023-dec 2023)"),
            },
            ExperienceItem {
                role: String::from("fullstack dev"),
                affiliation: String::from("adhd magazine"),
                time: String::from("(may 2023-aug 2023)"),
            },
            ExperienceItem {
                role: String::from("incubatee"),
                affiliation: String::from("ai camp"),
                time: String::from("(sep 2022-nov 2022)"),
            },
        ];

        Self {
            state: 0,
            experiences,
        }
    }

    pub fn build(&self) -> Table<'_> {
        let header = ["role", "affiliation", "time"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.experiences.iter().enumerate().map(|(i, data)| {
            let item = data.ref_array();

            let style_config = match i == self.state {
                true => Style::new().fg(Color::Rgb(0, 0, 0)).bg(Color::White),
                false => Style::new().fg(Color::Rgb(147, 147, 147)),
            };

            item.into_iter()
                .map(|content| Cell::from(content.as_str()))
                .collect::<Row>()
                .style(style_config)
                .height(1)
        });

        Table::new(
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
        }))
    }

    pub fn build_description(&self) -> Paragraph<'_> {
        let mut description: Vec<Line<'_>> = match self.state {
            0 => {
                vec![Line::from(vec![
                    Span::from("incoming summer 2026 under the tip program")
                        .fg(Color::Rgb(147, 147, 147)),
                ])]
            }
            1 => {
                vec![
                    Line::from(
                        vec![
                            Span::from("led a team of 4 to ship an irl dress to impress mobile app with 260+ users")
                        .fg(Color::Rgb(147, 147, 147))]
                        ),
                    Line::from(""),
                    Line::from(vec![
                        Span::from("notable highlights:")
                        .fg(Color::Rgb(147, 147, 147))
                    ]),
                    Line::from(vec![
                        Span::from("- achieved a 3x boost in DAU retention by analyzing user behavior patterns and implementing targeted push notifications").fg(Color::Rgb(147, 147, 147))
                    ])
                ]
            }
            2 => {
                vec![
                    Line::from(
                        vec![
                            Span::from("worked on the capital one empath dashboard on a team of 5")
                        .fg(Color::Rgb(147, 147, 147))
                        ]
                        ),
                    Line::from(""),
                    Line::from(vec![
                        Span::from("notable highlights:")
                        .fg(Color::Rgb(147, 147, 147))
                    ]),
                    Line::from(
                        vec![
                            Span::from("- created a digital enrollment status badge to help reduce Capital One agent call times by 12%")
                        .fg(Color::Rgb(147, 147, 147))
                        ]
                        )
                ]
            }
            3 => {
                vec![
                    Line::from(
                        vec![
                            Span::from("solo developed an event management mobile app for the society of women engineers at the university of michigan-dearborn's power conference")
                        .fg(Color::Rgb(147, 147, 147))]
                        ),
                    Line::from(""),
                    Line::from(
                        vec![
                            Span::from("features include qr code check-ins and a live agenda, message feed, and push notifications to keep attendees updated")
                        .fg(Color::Rgb(147, 147, 147))]
                        ),
                    Line::from(""),
                    Line::from(vec![
                        Span::from("notable highlights:")
                            .fg(Color::Rgb(147, 147, 147))
                    ]),
                    Line::from(vec![
                        Span::from("- deployed to the ios app store as 'power um-d'").fg(Color::Rgb(147, 147, 147))
                    ]),
                    Line::from(vec![
                        Span::from("- supported 80+ attendees").fg(Color::Rgb(147, 147, 147))
                    ])
                ]
            }
            4 => {
                vec![
                    Line::from(vec![
                        Span::from("built the michigan devfest 2023 website on a team of 8")
                            .fg(Color::Rgb(147, 147, 147)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::from("notable highlights:").fg(Color::Rgb(147, 147, 147)),
                    ]),
                    Line::from(vec![
                        Span::from("- website drove 300+ event attendees")
                            .fg(Color::Rgb(147, 147, 147)),
                    ]),
                ]
            }
            5 => {
                vec![
                    Line::from(vec![
                        Span::from(
                            "designed a blog platform to showcase Detroit's underground culture",
                        )
                        .fg(Color::Rgb(147, 147, 147)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::from("notable highlights:").fg(Color::Rgb(147, 147, 147)),
                    ]),
                    Line::from(vec![
                        Span::from("- engaged an audience of 2500+ followers")
                            .fg(Color::Rgb(147, 147, 147)),
                    ]),
                ]
            }
            6 => {
                vec![Line::from(vec![
                    Span::from("created gpt-3 wrapper that summarized videos, audio, and text as part of the 2023 ai camp incubator program").fg(Color::Rgb(147, 147, 147)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::from("notable highlights:").fg(Color::Rgb(147, 147, 147)),
                ]),
                Line::from(vec![
                    Span::from("- won $500 by placing 2nd place out of 21 other teams")
                        .fg(Color::Rgb(147, 147, 147)),
                ]),]
            }
            _ => {
                vec![]
            }
        };

        description.insert(0, Line::from(vec![Span::from("desc").fg(Color::White)]));

        Paragraph::new(description).wrap(Wrap { trim: true })
    }

    pub fn keyboard_event_handler(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Char('k') => {
                self.previous_experience();
                true
            }
            KeyCode::Char('j') => {
                self.next_experience();
                true
            }
            _ => false,
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
}
