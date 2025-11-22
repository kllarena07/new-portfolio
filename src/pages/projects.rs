use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Table, Wrap},
};

use crate::pages::{
    labels::{
        aws::{
            eventbridge_scheduler::EventBridgeScheduler, lambda::Lambda, s3::S3,
            sagemaker::SageMaker,
        },
        container::LabelContainer,
        fastapi::FastAPI,
        flask::Flask,
        groq::Groq,
        javascript::JavaScript,
        kinde_auth::KindeAuth,
        label::ColoredLabel,
        modal::Modal,
        nextjs::NextJS,
        pinecone::Pinecone,
        python::Python,
        retell_ai::RetellAI,
        supabase::Supabase,
        tailwind::Tailwind,
        typescript::TypeScript,
        websocket::WebSocket,
    },
    page::Page,
    style::{
        dimmed_selected_style, gray_span, gray_style, line_from_spans, selected_style, white_span,
    },
};

fn osc52(text: &str) {
    use base64::{Engine as _, engine::general_purpose};

    let encoded = general_purpose::STANDARD.encode(text.as_bytes());
    print!("\x1b]52;c;{}\x07", encoded);
    // Flush to ensure the sequence is sent immediately
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
}
struct ProjectItem {
    name: &'static str,
    link: &'static str,
    project_type: &'static str,
    prizes: Vec<&'static str>,
    description: Vec<&'static str>,
    technologies: Vec<ColoredLabel>,
}

impl ProjectItem {
    pub const fn ref_name(&self) -> [&str; 2] {
        [self.name, self.project_type]
    }
}

pub struct Projects {
    state: usize,
    current_link: String,
    projects: Vec<ProjectItem>,
    show_tooltip: bool,
    tooltip_end_tick: u64,
    current_tick: u64,
}

impl Page for Projects {
    fn title(&self) -> &str {
        "projects"
    }

    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
        // Split area into tooltip area and content area
        let [tooltip_area, content_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);

        // Render tooltip if active
        if self.show_tooltip {
            let tooltip_text = "✔ project link copied to clipboard";
            let tooltip_paragraph = Paragraph::new(tooltip_text)
                .style(ratatui::style::Style::new().fg(ratatui::style::Color::Green))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(tooltip_paragraph, tooltip_area);
        }

        let header = ["name", "project type"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.projects.iter().enumerate().map(|(i, data)| {
            let item = data.ref_name();

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

        // Calculate max width needed for project type column
        let max_project_type_len = self
            .projects
            .iter()
            .map(|p| p.project_type.len())
            .max()
            .unwrap_or(0) as u16;

        let table = Table::new(
            rows,
            [
                Constraint::Fill(1),
                Constraint::Length(max_project_type_len),
            ],
        )
        .header(header)
        .block(Block::new().padding(Padding {
            left: 1,
            right: 2,
            top: 0,
            bottom: 0,
        }));

        frame.render_widget(table, content_area);
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

        let project_item = &self.projects[self.state];
        let container = LabelContainer::new(&project_item.technologies);
        container.render(frame, tech_area);
    }

    fn keyboard_event_handler(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('k') | KeyCode::Up => {
                self.previous_project();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.next_project();
            }
            KeyCode::Enter => {
                osc52(&self.current_link);
                self.show_tooltip = true;
                self.tooltip_end_tick = self.current_tick + 38;
            }
            _ => {}
        }
    }

    fn nav_items(&self) -> Vec<Line<'static>> {
        vec![line_from_spans(vec![white_span(" ↵  "), gray_span("copy")])]
    }

    fn on_tick(&mut self, tick: u64) -> bool {
        self.current_tick = tick;
        if self.show_tooltip && tick >= self.tooltip_end_tick {
            self.show_tooltip = false;
        }
        true
    }
}

impl Projects {
    pub fn new() -> Self {
        let projects = vec![
            ProjectItem {
                name: "ecollm",
                link: "https://devpost.com/software/ecollm",
                prizes: vec!["🏆 best social impact"],
                description: vec![
                    "an adaptive ai model training tool for llms, optimized to minimize carbon footprint",
                    "",
                    "persists training epochs/checkpoints to aws s3 and orchestrates aws sagemaker jobs while dynamically rebalancing workloads across aws regions in real time to reduce carbon emissions",
                ],
                project_type: "hackathon (revolutionuc 2025)",
                technologies: vec![
                    NextJS::build(),
                    TypeScript::build(),
                    SageMaker::build(),
                    S3::build(),
                ],
            },
            ProjectItem {
                name: "dependapou",
                link: "https://devpost.com/software/depend-a-pou",
                prizes: vec![
                    "🏆 best software dev tool (sponsored by warp)",
                    "🏆 best use of modal (sponsored by modal labs)",
                ],
                description: vec![
                    "a developer tool that uses llms to ensure developers are shipping instead of maintaining",
                    "",
                    "scans codebases in seconds by parallelizing file checks for outdated deps/vulnerabilities with modal and groq",
                    "",
                    "auto-generates refactor prs and provides an insights dashboard for end‑to‑end visibility and control",
                ],
                project_type: "hackathon (columbia devfest 2025)",
                technologies: vec![
                    FastAPI::build(),
                    Groq::build(),
                    Modal::build(),
                    NextJS::build(),
                    Tailwind::build(),
                ],
            },
            ProjectItem {
                name: "ootd, outfit of the day",
                link: "https://devpost.com/software/ootd-outfit-of-the-day",
                prizes: vec!["🏆 zero waste award (sustainability track)"],
                description: vec![
                    "the all-in-one social media fashion app",
                    "",
                    "users can share their outfits, explore and vote on looks from others, try clothes on virtually, and shop their favorite pieces",
                ],
                project_type: "hackathon (msu spartahack x)",
                technologies: vec![
                    NextJS::build(),
                    Supabase::build(),
                    TypeScript::build(),
                    Tailwind::build(),
                ],
            },
            ProjectItem {
                name: "manny-bot",
                link: "https://github.com/kllarena07/safa-message-scheduler",
                prizes: vec![],
                description: vec![
                    "a web dashboard for scheduling discord announcements, built for the student association for filipino americans at um-dearborn",
                    "",
                    "the dashboard, locked behind authentication, is split into two sections: a composer that supports markdown input and file uploads and a previewer that renders the output",
                    "",
                    "upon scheduling, media assets are persisted to s3 and an eventbridge schedule is created with a payload (s3 urls + message body). at runtime, the schedule invokes a lambda function, which reads the payload and publishes to a discord webhook",
                ],
                project_type: "personal",
                technologies: vec![
                    NextJS::build(),
                    TypeScript::build(),
                    S3::build(),
                    Lambda::build(),
                    EventBridgeScheduler::build(),
                    KindeAuth::build(),
                ],
            },
            ProjectItem {
                name: "sheltr",
                link: "https://devpost.com/software/sheltr-xoz357",
                prizes: vec!["🏆 2nd place winner overall"],
                description: vec![
                    "a real-time crowdsourced disaster-management platform aimed to help both locals and responders during the january 2025 southern california wildfires",
                    "",
                    "users can view a live feed of nearby emergencies, submit location‑based disaster reports with key details, and see prioritized updates based on community engagement",
                ],
                project_type: "hackathon (waynehacks 3)",
                technologies: vec![
                    NextJS::build(),
                    Supabase::build(),
                    TypeScript::build(),
                    Tailwind::build(),
                ],
            },
            ProjectItem {
                name: "youtube copilot",
                link: "https://github.com/kllarena07/yt-copilot",
                prizes: vec!["🏆 5th place winner overall"],
                description: vec![
                    "a chrome extension that enables ai conversations with youtube videos",
                    "",
                    "leverages retrieval‑augmented generation over the video transcript and the active frame to provide context-aware answers to user prompts during playback",
                ],
                project_type: "hackathon (intel ai pc pilot program)",
                technologies: vec![
                    Pinecone::build(),
                    Flask::build(),
                    Python::build(),
                    JavaScript::build(),
                    WebSocket::build(),
                ],
            },
            ProjectItem {
                name: "safety blanket",
                link: "https://devpost.com/software/safety-blanket-vyp089",
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
                technologies: vec![
                    NextJS::build(),
                    Flask::build(),
                    WebSocket::build(),
                    RetellAI::build(),
                    Python::build(),
                    Supabase::build(),
                    TypeScript::build(),
                    Tailwind::build(),
                ],
            },
        ];

        Self {
            state: 0,
            current_link: String::from(projects[0].link),
            projects,
            show_tooltip: false,
            tooltip_end_tick: 0,
            current_tick: 0,
        }
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
            self.change_current_link();
        }
    }

    fn next_project(&mut self) {
        if self.state < self.projects.len() - 1 {
            self.state += 1;
            self.change_current_link();
        }
    }

    fn change_current_link(&mut self) {
        self.current_link = String::from(self.projects[self.state].link);
    }
}
