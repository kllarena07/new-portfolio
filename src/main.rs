use crossterm::event::KeyCode;
use image::ImageReader;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Padding, Paragraph, Wrap,
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

    let mut app = App {
        running: true,
        selected_page: 0,
        count: 0,
        links: links.clone(),
        pages,
        all_frames,
        max_frames,
        about_page_state: 0,
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

struct App<'a> {
    running: bool,
    selected_page: usize,
    count: usize,
    links: Vec<ContactLink>,
    pages: Vec<&'a str>,
    all_frames: Vec<Vec<Vec<[u8; 3]>>>,
    max_frames: usize,
    about_page_state: usize,
    current_link: String,
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
        // frame.render_widget(
        //     Block::new()
        //         .fg(Color::Blue)
        //         .title("Right")
        //         .borders(Borders::ALL),
        //     right_area,
        // );

        let menu_widget = self.build_menu_widget();
        let about_page: Paragraph = self.build_about_page();
        let canvas = Canvas::default()
            .marker(ratatui::symbols::Marker::HalfBlock)
            .x_bounds([0.0, 112.0])
            .y_bounds([0.0, 112.0])
            .paint(|ctx| {
                // Draw pixels from the current frame
                for (y, row) in current_frame.iter().enumerate() {
                    for (x, pixel) in row.iter().enumerate() {
                        let canvas_x = x as f64;
                        let canvas_y = (111_usize.saturating_sub(y)) as f64;

                        ctx.draw(&Points {
                            coords: &[(canvas_x, canvas_y)],
                            color: ratatui::style::Color::Rgb(pixel[0], pixel[1], pixel[2]),
                        });
                    }
                }
            });

        frame.render_widget(menu_widget, menu_area);
        frame.render_widget(about_page, center_area);
        frame.render_widget(canvas, right_area);
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
                .border_style(Style::new().fg(Color::DarkGray))
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
