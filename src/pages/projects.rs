use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Cell, Padding, Paragraph, Row, Table, Wrap},
};

use crate::pages::page::Page;

struct ProjectItem {
    name: String,
    project_type: String,
    prizes: Vec<String>,
    description: String,
    technologies: Vec<String>,
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
                true => Style::new().fg(Color::Rgb(0, 0, 0)).bg(Color::White),
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
        description.insert(0, Line::from(vec![Span::from("desc").fg(Color::White)]));

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
                description: String::from(
                    "an adaptive ai model training tool for llms, optimized to minimize carbon footprint by leveraging aws sagemaker and dynamically shifting data centers to reduce emissions through a custom integration, while using aws s3 to ensure fast, reliable data transfers across aws regions",
                ),
                technologies: vec![
                    String::from("aws sagemaker"),
                    String::from("aws s3"),
                    String::from("nextjs"),
                    String::from("typescript"),
                ],
                project_type: String::from("hackathon (revolutionuc 2025)"),
            },
            ProjectItem {
                name: String::from("dependapou"),
                prizes: vec![
                    String::from("🏆 best software dev tool (sponsored by warp)"),
                    String::from("🏆 best use of modal (Sponsored by modal labs)"),
                ],
                description: String::from(
                    "a developer tool that uses generative ai to streamline maintenance by scanning your codebase, detecting outdated dependencies or vulnerabilities, and automatically generating prs with refactored code, while also providing a dashboard with insights into the refactor process for full visibility and control",
                ),
                technologies: vec![
                    String::from("fastapi"),
                    String::from("groq"),
                    String::from("modal"),
                    String::from("nextjs"),
                    String::from("typescript"),
                ],
                project_type: String::from("hackathon (columbia devfest 2025)"),
            },
            ProjectItem {
                name: String::from("ootd, outfit of the day"),
                prizes: vec![String::from("🏆 zero waste award (sustainability track)")],
                description: String::from(
                    "a social media fashion app that lets users post their outfits, explore and vote on looks from others, try clothes on virtually, and shop their favorite pieces, all within the app",
                ),
                technologies: vec![
                    String::from("nextjs"),
                    String::from("typescript"),
                    String::from("supabase"),
                ],
                project_type: String::from("hackathon (msu spartahack x)"),
            },
            ProjectItem {
                name: String::from("manny-bot"),
                prizes: vec![],
                description: String::from(
                    "noticing how my fellow e-board members could benefit from a way to streamline discord announcement by scheduling them, i took matters into my own hands by developing such a mechanism from scratch. on the web dashboard, announcements can be previewed using a built-in markdown previewer and then scheduled to be sent out",
                ),
                technologies: vec![
                    String::from("nextjs"),
                    String::from("typescript"),
                    String::from("aws s3"),
                    String::from("aws lambda"),
                    String::from("aws eventbridge scheduler"),
                    String::from("kinde auth"),
                    String::from("discord api"),
                ],
                project_type: String::from("personal"),
            },
            ProjectItem {
                name: String::from("sheltr"),
                prizes: vec![String::from("🏆 2nd place winner overall")],
                description: String::from(
                    "a crowdsourced disaster management platform that gives real-time incident updates and lets users report local emergencies through a community-driven system. users can see a live feed of emergencies near them, add location-based disaster reports with key details, and rely on a prioritization system that highlights the most critical updates based on community engagement, helping everyone stay aware and respond faster",
                ),
                technologies: vec![
                    String::from("fastapi"),
                    String::from("groq"),
                    String::from("modal"),
                    String::from("nextjs"),
                    String::from("typescript"),
                ],
                project_type: String::from("hackathon (waynehacks 3)"),
            },
            ProjectItem {
                name: String::from("youtube copilot"),
                prizes: vec![String::from("🏆 5th place winner overall")],
                description: String::from(
                    "developed a chrome extension that works directly with youtube, using retrieval-augmented generation to let users have real-time, ai-powered conversations with videos by asking questions and getting relevant answers as they watch",
                ),
                technologies: vec![
                    String::from("pinecone"),
                    String::from("flask"),
                    String::from("python"),
                    String::from("javascript"),
                    String::from("websockets"),
                    String::from("html/css"),
                ],
                project_type: String::from("hackathon (intel ai pc pilot program)"),
            },
            ProjectItem {
                name: String::from("safety blanket"),
                prizes: vec![],
                description: String::from(
                    "a virtual companion app that enhances personal security and emotional support by featuring an ai Companion for real-time phone calls to provide comfort and reduce anxiety, real-time text chat check-ins that escalate to location sharing if the user is unresponsive, and a safety timer that triggers emergency alerts if not deactivated in time",
                ),
                technologies: vec![
                    String::from("nextjs"),
                    String::from("typescript"),
                    String::from("websocket"),
                    String::from("retell ai"),
                    String::from("pythonj"),
                    String::from("flask"),
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
            final_vec.push(Line::from(prize.to_string()).fg(Color::Rgb(147, 147, 147)));
        }

        if project_item.prizes.len() > 0 {
            final_vec.push(Line::from(""));
        }
        final_vec
            .push(Line::from(project_item.description.to_string()).fg(Color::Rgb(147, 147, 147)));

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
