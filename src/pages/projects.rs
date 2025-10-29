use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Table, Wrap},
};

use crate::pages::page::Page;
use crate::pages::style::{gray_span, gray_style, line_from_spans, selected_style, white_span};

struct ProjectItem {
    name: &'static str,
    project_type: &'static str,
    prizes: Vec<&'static str>,
    description: Vec<&'static str>,
}

impl ProjectItem {
    pub const fn ref_name(&self) -> [&str; 2] {
        [self.name, self.project_type]
    }
}

pub struct Projects {
    state: usize,
    projects: Vec<ProjectItem>,
}

impl Page for Projects {
    fn title(&self) -> &str {
        "projects"
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let header = ["name", "project type"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.projects.iter().enumerate().map(|(i, data)| {
            let item = data.ref_name();

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

        let table = Table::new(rows, [Constraint::Fill(1), Constraint::Fill(2)])
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
                self.previous_project();
            }
            KeyCode::Char('j') => {
                self.next_project();
            }
            _ => {}
        }
    }
}

impl Projects {
    pub fn new() -> Self {
        let projects = vec![
            ProjectItem {
                name: "ecollm",
                prizes: vec!["🏆 best social impact"],
                description: vec![
                    "an adaptive ai model training tool for llms, optimized to minimize carbon footprint",
                    "",
                    "persists training epochs/checkpoints to aws s3 and orchestrates aws sagemaker jobs while dynamically rebalancing workloads across aws regions in real time to reduce carbon emissions",
                ],
                project_type: "hackathon (revolutionuc 2025)",
            },
            ProjectItem {
                name: "dependapou",
                prizes: vec![
                    "🏆 best software dev tool (sponsored by warp)",
                    "🏆 best use of modal (Sponsored by modal labs)",
                ],
                description: vec![
                    "a developer tool that uses llms to ensure developers are shipping instead of maintaining",
                    "",
                    "scans codebases in seconds by parallelizing file checks for outdated deps/vulnerabilities with modal and groq",
                    "",
                    "auto-generates refactor prs and provides an insights dashboard for end‑to‑end visibility and control",
                ],
                project_type: "hackathon (columbia devfest 2025)",
            },
            ProjectItem {
                name: "ootd, outfit of the day",
                prizes: vec!["🏆 zero waste award (sustainability track)"],
                description: vec![
                    "the all-in-one social media fashion app",
                    "",
                    "users can share their outfits, explore and vote on looks from others, try clothes on virtually, and shop their favorite pieces",
                ],
                project_type: "hackathon (msu spartahack x)",
            },
            ProjectItem {
                name: "manny-bot",
                prizes: vec![],
                description: vec![
                    "a web dashboard for scheduling discord announcements, built for the student association for filipino americans at um-dearborn",
                    "",
                    "the dashboard, locked behind authentication, is split into two sections: composer and previewer. the composer supports markdown input and file uploads while the previewer renders the output",
                    "",
                    "upon scheduling, media assets are persisted to s3 and an eventbridge schedule is created with a payload (s3 urls + message body). at runtime, the schedule invokes a lambda function, which reads the payload and publishes to a discord webhook",
                ],
                project_type: "personal",
            },
            ProjectItem {
                name: "sheltr",
                prizes: vec!["🏆 2nd place winner overall"],
                description: vec![
                    "a real-time crowdsourced disaster-management platform aimed to help both locals and responders during the january 2025 southern california wildfires",
                    "",
                    "users can view a live feed of nearby emergencies, submit location‑based disaster reports with key details, and see prioritized updates based on community engagement",
                ],
                project_type: "hackathon (waynehacks 3)",
            },
            ProjectItem {
                name: "youtube copilot",
                prizes: vec!["🏆 5th place winner overall"],
                description: vec![
                    "a chrome extension that enables ai conversations with youtube videos",
                    "",
                    "leverages retrieval‑augmented generation (rag) over the video transcript and the active frame to provide context-aware answers to user prompts during playback",
                ],
                project_type: "hackathon (intel ai pc pilot program)",
            },
            ProjectItem {
                name: "safety blanket",
                prizes: vec![],
                description: vec![
                    "a virtual companion app built to provide security for women traveling at night",
                    "",
                    "the app offers several ai-driven safety measures that support automated escalation to authorities:",
                    "",
                    "1. real-time text check-ins",
                    "2. a safety timer with countdown/expiry events",
                    "3. voice-call interface that simulates talking to a real person, with safe-word detection",
                ],
                project_type: "hackathon (venushacks 2024)",
            },
        ];

        Self { state: 0, projects }
    }

    fn get_description(&self) -> Vec<Line<'_>> {
        let project_index = self.state;
        let mut final_vec: Vec<Line<'_>> = vec![];
        let project_item = &self.projects[project_index];

        for prize in &project_item.prizes {
            final_vec.push(line_from_spans(vec![gray_span(&prize)]));
        }

        if project_item.prizes.len() > 0 {
            final_vec.push(Line::from(""));
        }

        for desc_part in &project_item.description {
            final_vec.push(line_from_spans(vec![gray_span(&desc_part)]));
        }

        final_vec
    }

    fn previous_project(&mut self) {
        if self.state > 0 {
            self.state -= 1;
        }
    }

    fn next_project(&mut self) {
        if self.state < self.projects.len() - 1 {
            self.state += 1;
        }
    }
}
