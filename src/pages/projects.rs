use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Table, Wrap},
};

use crate::pages::page::Page;
use crate::pages::style::{gray_span, selected_style, white_span};

struct ProjectItem {
    name: String,
    project_type: String,
    prizes: Vec<String>,
    description: Vec<String>,
}

impl ProjectItem {
    pub const fn ref_name(&self) -> [&String; 2] {
        [&self.name, &self.project_type]
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
                false => Style::new().fg(Color::Rgb(147, 147, 147)),
            };

            item.into_iter()
                .map(|content| Cell::from(content.as_str()))
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
        description.insert(0, Line::from(vec![white_span("desc")]));

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
                name: String::from("ecollm"),
                prizes: vec![String::from("🏆 best social impact")],
                description: vec![
                    String::from(
                        "an adaptive ai model training tool for llms, optimized to minimize carbon footprint",
                    ),
                    String::from(""),
                    String::from(
                        "persists training epochs/checkpoints to aws s3 and orchestrates aws sagemaker jobs while dynamically rebalancing workloads across aws regions in real time to reduce carbon emissions",
                    ),
                ],
                project_type: String::from("hackathon (revolutionuc 2025)"),
            },
            ProjectItem {
                name: String::from("dependapou"),
                prizes: vec![
                    String::from("🏆 best software dev tool (sponsored by warp)"),
                    String::from("🏆 best use of modal (Sponsored by modal labs)"),
                ],
                description: vec![
                    String::from(
                        "a developer tool that uses llms to ensure developers are shipping instead of maintaining",
                    ),
                    String::from(""),
                    String::from(
                        "scans codebases in seconds by parallelizing file checks for outdated deps/vulnerabilities with modal and groq",
                    ),
                    String::from(""),
                    String::from(
                        "auto-generates refactor prs and provides an insights dashboard for end‑to‑end visibility and control",
                    ),
                ],
                project_type: String::from("hackathon (columbia devfest 2025)"),
            },
            ProjectItem {
                name: String::from("ootd, outfit of the day"),
                prizes: vec![String::from("🏆 zero waste award (sustainability track)")],
                description: vec![
                    String::from("the all-in-one social media fashion app"),
                    String::from(""),
                    String::from(
                        "users can share their outfits, explore and vote on looks from others, try clothes on virtually, and shop their favorite pieces",
                    ),
                ],
                project_type: String::from("hackathon (msu spartahack x)"),
            },
            ProjectItem {
                name: String::from("manny-bot"),
                prizes: vec![],
                description: vec![
                    String::from(
                        "a web dashboard for scheduling discord announcements, built for the student association for filipino americans at um-dearborn",
                    ),
                    String::from(""),
                    String::from(
                        "the dashboard, locked behind authentication, is split into two sections: composer and previewer. the composer supports markdown input and file uploads while the previewer renders the output",
                    ),
                    String::from(""),
                    String::from(
                        "upon scheduling, media assets are persisted to s3 and an eventbridge schedule is created with a payload (s3 urls + message body). at runtime, the schedule invokes a lambda function, which reads the payload and publishes to a discord webhook",
                    ),
                ],
                project_type: String::from("personal"),
            },
            ProjectItem {
                name: String::from("sheltr"),
                prizes: vec![String::from("🏆 2nd place winner overall")],
                description: vec![
                    String::from(
                        "a real-time crowdsourced disaster-management platform aimed to help both locals and responders during the january 2025 southern california wildfires",
                    ),
                    String::from(""),
                    String::from(
                        "users can view a live feed of nearby emergencies, submit location‑based disaster reports with key details, and see prioritized updates based on community engagement",
                    ),
                ],
                project_type: String::from("hackathon (waynehacks 3)"),
            },
            ProjectItem {
                name: String::from("youtube copilot"),
                prizes: vec![String::from("🏆 5th place winner overall")],
                description: vec![
                    String::from(
                        "a chrome extension that enables ai conversations with youtube videos",
                    ),
                    String::from(""),
                    String::from(
                        "leverages retrieval‑augmented generation (rag) over the video transcript and the active frame to provide context-aware answers to user prompts during playback",
                    ),
                ],
                project_type: String::from("hackathon (intel ai pc pilot program)"),
            },
            ProjectItem {
                name: String::from("safety blanket"),
                prizes: vec![],
                description: vec![
                    String::from(
                        "a virtual companion app built to provide security for women traveling at night",
                    ),
                    String::from(""),
                    String::from(
                        "the app offers several ai-driven safety measures that support automated escalation to authorities:",
                    ),
                    String::from(""),
                    String::from("1. real-time text check-ins"),
                    String::from("2. a safety timer with countdown/expiry events"),
                    String::from(
                        "3. voice-call interface that simulates talking to a real person, with safe-word detection",
                    ),
                ],
                project_type: String::from("hackathon (venushacks 2024)"),
            },
        ];

        Self { state: 0, projects }
    }

    fn get_description(&self) -> Vec<Line<'_>> {
        let project_index = self.state;
        let mut final_vec: Vec<Line<'_>> = vec![];
        let project_item = &self.projects[project_index];

        for prize in &project_item.prizes {
            final_vec.push(Line::from(vec![gray_span(&prize)]));
        }

        if project_item.prizes.len() > 0 {
            final_vec.push(Line::from(""));
        }

        for desc_part in &project_item.description {
            final_vec.push(Line::from(vec![gray_span(&desc_part)]));
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
