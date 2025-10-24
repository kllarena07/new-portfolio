use crossterm::event::KeyCode;
use image::ImageReader;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, List, ListItem, Padding, Paragraph, Row, Table, Wrap,
        canvas::{Canvas, Points},
    },
};
use std::fs;
use std::{io, sync::mpsc, thread, time::Duration};

fn get_all_frames_rgb_vals() -> Vec<Vec<Vec<[u8; 3]>>> {
    const LIMIT_TO_10_FRAMES: bool = true; // Set to true to only load first 10 frames
    let mut all_frames = Vec::new();

    // Read all frame files from hikari directory
    let mut frame_files = Vec::new();
    if let Ok(entries) = fs::read_dir("./hikari-dance") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "png" || extension == "jpg" || extension == "jpeg" {
                        if let Some(_file_name) = path.file_name() {
                            frame_files.push(path.clone());
                        }
                    }
                }
            }
        }
    }

    // Sort the files numerically by extracting frame numbers
    frame_files.sort_by(|a, b| {
        let extract_frame_number = |path: &std::path::PathBuf| -> i32 {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(|name| name.strip_prefix("frame_"))
                .and_then(|num_str| num_str.parse::<i32>().ok())
                .unwrap_or(0)
        };

        let a_num = extract_frame_number(a);
        let b_num = extract_frame_number(b);
        a_num.cmp(&b_num)
    });

    // Debug: Print first few frame names to verify ordering
    println!("Frame loading order (first 10):");
    for (i, path) in frame_files.iter().take(10).enumerate() {
        println!("{}: {}", i, path.file_name().unwrap().to_string_lossy());
    }

    // Process each frame
    let frames_to_process = if LIMIT_TO_10_FRAMES {
        frame_files.into_iter().take(10).collect()
    } else {
        frame_files
    };

    for frame_path in frames_to_process {
        if let Ok(img) = ImageReader::open(&frame_path) {
            if let Ok(decoded_img) = img.decode() {
                // Resize to square dimensions
                let resized_img =
                    decoded_img.resize(112, 112, image::imageops::FilterType::Lanczos3);
                let rgb_img = resized_img.to_rgb8();
                let (width, height) = rgb_img.dimensions();

                // Create 2D array to store RGB values for this frame
                let mut pixel_rgb_val_map: Vec<Vec<[u8; 3]>> = Vec::with_capacity(height as usize);

                for y in 0..height {
                    let mut row: Vec<[u8; 3]> = Vec::with_capacity(width as usize);
                    for x in 0..width {
                        let pixel = rgb_img.get_pixel(x, y);
                        row.push([pixel[0], pixel[1], pixel[2]]);
                    }
                    pixel_rgb_val_map.push(row);
                }

                all_frames.push(pixel_rgb_val_map);
            }
        }
    }

    all_frames
}

fn main() -> io::Result<()> {
    let all_frames = get_all_frames_rgb_vals();
    let max_frames = all_frames.len();
    println!(
        "width: {}, height: {}",
        all_frames[0][0].len(),
        all_frames[0].len()
    );
    let pages = vec!["about", "experience", "projects", "leadership"];

    let links: Vec<ContactLink> = vec![
        ContactLink {
            display_text: String::from("twitter"),
            link: String::from("https://x.com/krayondev"),
        },
        ContactLink {
            display_text: String::from("linkedin"),
            link: String::from("https://www.linkedin.com/in/kllarena07/"),
        },
        ContactLink {
            display_text: String::from("github"),
            link: String::from("https://github.com/kllarena07"),
        },
        ContactLink {
            display_text: String::from("email"),
            link: String::from("mailto:kieran.llarena@gmail.com"),
        },
    ];

    let experience = vec![
        Experience {
            role: String::from("swe intern"),
            affiliation: String::from("capital one"),
            time: String::from("(jun 2026-aug 2026)"),
        },
        Experience {
            role: String::from("ceo / cto"),
            affiliation: String::from("ootd"),
            time: String::from("(mar 2025-oct 2025)"),
        },
        Experience {
            role: String::from("swe intern"),
            affiliation: String::from("capital one"),
            time: String::from("(jun 2025-aug 2025)"),
        },
        Experience {
            role: String::from("mobile app dev"),
            affiliation: String::from("swe, um-dearborn"),
            time: String::from("(feb 2025-mar 2025)"),
        },
        Experience {
            role: String::from("frontend dev"),
            affiliation: String::from("gdsc, um-dearborn"),
            time: String::from("(nov 2023-dec 2023)"),
        },
        Experience {
            role: String::from("fullstack dev"),
            affiliation: String::from("adhd magazine"),
            time: String::from("(may 2023-aug 2023)"),
        },
        Experience {
            role: String::from("incubatee"),
            affiliation: String::from("ai camp"),
            time: String::from("(sep 2022-nov 2022)"),
        },
    ];

    let mut app = App {
        running: true,
        selected_page: 0,
        count: 0,
        links: links.clone(),
        experience,
        pages,
        all_frames,
        max_frames,
        about_page_state: 0,
        experience_page_state: 0,
        current_link: links[0].link.clone(),
    };

    let mut terminal = ratatui::init();

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let tx_to_counter_events = event_tx.clone();
    let max_frames_for_thread = max_frames;
    thread::spawn(move || {
        run_background_thread(tx_to_counter_events, max_frames_for_thread);
    });

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();
    app_result
}

enum Event {
    Input(crossterm::event::KeyEvent),
    Counter(usize),
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

fn run_background_thread(tx: mpsc::Sender<Event>, max_frames: usize) {
    let framerate = 30;
    let frame_duration = Duration::from_millis(1000 / framerate);

    loop {
        for count in 0..max_frames {
            tx.send(Event::Counter(count)).unwrap();
            thread::sleep(frame_duration);
        }
    }
}

#[derive(Clone)]
struct ContactLink {
    display_text: String,
    link: String,
}

struct Experience {
    role: String,
    affiliation: String,
    time: String,
}

impl Experience {
    const fn ref_array(&self) -> [&String; 3] {
        [&self.role, &self.affiliation, &self.time]
    }
}

struct App<'a> {
    running: bool,
    selected_page: usize,
    count: usize,
    max_frames: usize,
    about_page_state: usize,
    experience_page_state: usize,
    current_link: String,
    links: Vec<ContactLink>,
    experience: Vec<Experience>,
    pages: Vec<&'a str>,
    all_frames: Vec<Vec<Vec<[u8; 3]>>>,
}

impl<'a> App<'a> {
    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while self.running {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Counter(count) => self.count = count,
            }

            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        if self.all_frames.is_empty() {
            return;
        }

        // Get the current frame based on the counter
        let current_frame_index = self.count % self.max_frames;
        let current_frame = &self.all_frames[current_frame_index];

        let [vertical_area] = Layout::vertical([Constraint::Percentage(35)])
            .flex(Flex::Center)
            .areas(frame.area());
        let [left_area, center_area, right_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Max(80),
            Constraint::Fill(1),
        ])
        .flex(Flex::Center)
        .areas(vertical_area);
        let menu_height: u16 = (self.pages.len() + 2) as u16;
        let [menu_area] = Layout::vertical([Constraint::Max(menu_height)]).areas(left_area);
        let [vcanvas_area] = Layout::vertical([Constraint::Max(15)]).areas(right_area);
        let [canvas_area] = Layout::horizontal([Constraint::Max(50)]).areas(vcanvas_area);

        // frame.render_widget(
        //     Block::new()
        //         .fg(Color::Red)
        //         .title("Left")
        //         .borders(Borders::ALL),
        //     left_area,
        // );
        // frame.render_widget(
        //     Block::new()
        //         .fg(Color::Green)
        //         .title("Center")
        //         .borders(Borders::ALL),
        //     center_area,
        // );
        frame.render_widget(
            Block::new()
                .fg(Color::Blue)
                .title("Right")
                .borders(Borders::ALL),
            canvas_area,
        );

        let menu_widget = self.build_menu_widget();
        match self.selected_page {
            0 => {
                let about_page = self.build_about_page();
                frame.render_widget(about_page, center_area);
            }
            1 => {
                let experience_page = self.build_experience_page();
                frame.render_widget(experience_page, center_area);
            }
            2 => {
                let projects_page = self.build_projects_page();
                frame.render_widget(projects_page, center_area);
            }
            3 => {
                let leadership_page = self.build_leadership_page();
                frame.render_widget(leadership_page, center_area);
            }
            _ => {}
        }

        // Canvas bounds should match terminal character dimensions
        // For HalfBlock marker, each terminal cell is 1 wide and 2 tall in canvas coordinates
        let canvas_width = canvas_area.width as f64;
        let canvas_height = (canvas_area.height * 2) as f64; // HalfBlock doubles vertical resolution

        let canvas = Canvas::default()
            .marker(ratatui::symbols::Marker::HalfBlock)
            .x_bounds([0.0, canvas_width])
            .y_bounds([0.0, canvas_height])
            .paint(|ctx| {
                // Stretch the 112x112 frame to fill the entire canvas area
                let frame_width = 112.0;
                let frame_height = 112.0;

                // Draw pixels from the current frame, mapping each pixel to fill the canvas
                for (y, row) in current_frame.iter().enumerate() {
                    for (x, pixel) in row.iter().enumerate() {
                        // Map frame coordinates directly to canvas coordinates
                        let canvas_x = (x as f64 / frame_width) * canvas_width;
                        let canvas_y = canvas_height - ((y as f64 / frame_height) * canvas_height);

                        ctx.draw(&Points {
                            coords: &[(canvas_x, canvas_y)],
                            color: ratatui::style::Color::Rgb(pixel[0], pixel[1], pixel[2]),
                        });
                    }
                }
            });

        frame.render_widget(menu_widget, menu_area);

        match self.selected_page {
            0 => frame.render_widget(canvas, canvas_area),
            1 => {
                let description = self.build_experience_description();
                frame.render_widget(description, canvas_area)
            }
            _ => {}
        };
    }

    fn previous_page(&mut self) {
        if self.selected_page > 0 {
            self.selected_page -= 1;
        }
    }

    fn next_page(&mut self) {
        if self.selected_page + 1 < self.pages.len() {
            if self.selected_page == 0 {
                self.current_link = String::from("");
            }
            self.selected_page += 1;
        }
    }

    fn previous_link(&mut self) {
        if self.selected_page != 0 {
            return;
        }

        if self.about_page_state > 0 {
            self.about_page_state -= 1;
            self.current_link = self.links[self.about_page_state].link.to_owned();
        }
    }

    fn next_link(&mut self) {
        if self.selected_page != 0 {
            return;
        }

        if self.about_page_state < self.links.len() - 1 {
            self.about_page_state += 1;
            self.current_link = self.links[self.about_page_state].link.to_owned();
        }
    }

    fn previous_experience(&mut self) {
        if self.selected_page != 1 {
            return;
        }

        if self.experience_page_state > 0 {
            self.experience_page_state -= 1;
        }
    }

    fn next_experience(&mut self) {
        if self.selected_page != 1 {
            return;
        }

        if self.experience_page_state < self.experience.len() - 1 {
            self.experience_page_state += 1;
        }
    }

    fn open_current_link(&mut self) {
        if !&self.current_link.is_empty() {
            open::that(&self.current_link).unwrap();
        }
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => {
                self.running = false;
            }
            KeyCode::Left => self.previous_link(),
            KeyCode::Right => self.next_link(),
            KeyCode::Up => self.previous_page(),
            KeyCode::Down => self.next_page(),
            KeyCode::Enter => self.open_current_link(),
            KeyCode::Char('k') => self.previous_experience(),
            KeyCode::Char('j') => self.next_experience(),
            _ => {}
        }

        Ok(())
    }

    fn build_about_page(&self) -> Paragraph<'_> {
        let line_1 = Line::from(vec![
            Span::styled(
                "hey! my name is ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "kieran llarena",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_2 = Line::from(vec![
            Span::styled(
                "im currently studying ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "computer science ",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
            Span::styled("at the ", Style::default().fg(Color::Rgb(147, 147, 147))),
            Span::styled(
                "university of michigan-dearborn",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_3 = Line::from(vec![
            Span::styled(
                "my expected graduation date is ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled("may 2027", Style::default().fg(Color::Rgb(255, 255, 255))),
        ]);

        let line_4 = Line::from(vec![
            Span::styled(
                "i thrive best in environments that value ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "high velocity ",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
            Span::styled("and ", Style::default().fg(Color::Rgb(147, 147, 147))),
            Span::styled(
                "strong ownership",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_5 = Line::from(vec![
            Span::styled(
                "my background is rooted in ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "web and mobile fullstack development",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_6 = Line::from(vec![
            Span::styled(
                "im currently exploring ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "systems programming",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
            Span::styled(
                ", specifically working with ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "embedded Rust on microcontrollers",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        let line_items: Vec<Span> = (0..(self.links.len() * 2) - 1)
            .map(move |index| {
                if (index + 1) % 2 == 0 {
                    return Span::styled(" - ", Style::default().fg(Color::Rgb(147, 147, 147)));
                }

                let style_config = match index / 2 == self.about_page_state {
                    true => Style::default().fg(Color::Rgb(0, 255, 251)).underlined(),
                    false => Style::default().fg(Color::Rgb(147, 147, 147)),
                };

                let display_text = self.links[index / 2].display_text.to_owned();
                let _link = self.links[index / 2].link.to_owned();

                Span::styled(display_text, style_config)
            })
            .collect();

        let links_line = Line::from(line_items);

        Paragraph::new(vec![
            line_1,
            Line::from(""),
            line_2,
            Line::from(""),
            line_3,
            Line::from(""),
            line_4,
            Line::from(""),
            line_5,
            Line::from(""),
            line_6,
            Line::from(""),
            links_line,
        ])
        .block(Block::new().padding(Padding {
            left: 1,
            right: 2,
            top: 0,
            bottom: 0,
        }))
        .wrap(Wrap { trim: true })
    }

    fn build_experience_page(&self) -> Table<'_> {
        let header = ["role", "affiliation", "time"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.experience.iter().enumerate().map(|(i, data)| {
            let item = data.ref_array();

            let style_config = match i == self.experience_page_state {
                true => Style::new().fg(Color::Rgb(0, 0, 0)).bg(Color::White),
                false => Style::new().fg(Color::Rgb(147, 147, 147)),
            };

            item.into_iter()
                .map(|content| Cell::from(content.as_str()))
                .collect::<Row>()
                .style(style_config)
                .height(1)
        });

        let final_table = Table::new(
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

        final_table
    }

    fn build_experience_description(&self) -> Paragraph<'_> {
        let mut description: Vec<Line<'_>> = match self.experience_page_state {
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

    fn build_projects_page(&self) -> Block<'_> {
        Block::new()
            .borders(Borders::ALL)
            .title("Projects")
            .border_style(Color::Red)
    }

    fn build_leadership_page(&self) -> Block<'_> {
        Block::new()
            .borders(Borders::ALL)
            .title("Leadership")
            .border_style(Color::Green)
    }

    fn build_menu_widget(&self) -> List<'_> {
        let menu_items: Vec<ListItem> = (0..self.pages.len())
            .map(move |index| {
                let item_content = match index == self.selected_page {
                    true => format!("[ {} ]", self.pages[index]),
                    false => self.pages[index].to_owned(),
                };

                let span = match index == self.selected_page {
                    true => {
                        Span::styled(item_content, Style::default().fg(Color::Rgb(255, 255, 255)))
                    }
                    false => {
                        Span::styled(item_content, Style::default().fg(Color::Rgb(147, 147, 147)))
                    }
                };

                ListItem::new(span.bold().into_right_aligned_line())
            })
            .collect();

        let final_list = List::new(menu_items).block(
            Block::new()
                .borders(Borders::RIGHT)
                .border_set(symbols::border::ONE_EIGHTH_TALL)
                .border_style(Style::new().fg(Color::Rgb(147, 147, 147)))
                .padding(Padding {
                    top: 1,
                    bottom: 1,
                    right: 2,
                    left: 0,
                }),
        );

        final_list
    }
}
