use crossterm::event::KeyCode;
use image::ImageReader;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
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
    for frame_path in frame_files {
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

    let mut app = App {
        running: true,
        selected_page: 0,
        count: 0,
        all_frames,
        max_frames,
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

struct App {
    running: bool,
    selected_page: usize,
    count: usize,
    all_frames: Vec<Vec<Vec<[u8; 3]>>>,
    max_frames: usize,
}

impl App {
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

        fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
            let [area] = Layout::horizontal([horizontal])
                .flex(Flex::Center)
                .areas(area);
            let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
            area
        }

        let portfolio_area = center(
            frame.area(),
            Constraint::Length(150),
            Constraint::Length(25),
        );

        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                // must add up to 100
                Constraint::Max(17),
                Constraint::Percentage(57),
                Constraint::Min(26),
            ])
            .split(portfolio_area);

        // let menu = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints(vec![Constraint::Max(6)])
        //     .split(outer_layout[0]);

        // frame.render_widget(

        //     menu[0],
        // );

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
        frame.render_widget(canvas, outer_layout[2]);
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => {
                self.running = false;
            }
            KeyCode::Up => {
                if self.selected_page > 0 {
                    self.selected_page -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_page < 3 {
                    self.selected_page += 1;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn build_about_page(&self) -> Paragraph {
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
            Span::styled(
                ". my expected graduation date is ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled("may 2027", Style::default().fg(Color::Rgb(255, 255, 255))),
        ]);

        let line_3 = Line::from(vec![
            Span::styled(
                "interested in working on teams that value ",
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

        let line_4 = Line::from(vec![
            Span::styled(
                "i have extensive experience in fullstack development, both in web and mobile. i particularly enjoy ",
                Style::default().fg(Color::Rgb(147, 147, 147)),
            ),
            Span::styled(
                "designing infrastructure that scales reliably and cost-effectively",
                Style::default().fg(Color::Rgb(255, 255, 255)),
            ),
        ]);

        Paragraph::new(vec![
            line_1,
            Line::from(""),
            line_2,
            Line::from(""),
            line_3,
            Line::from(""),
            line_4,
        ])
        .block(Block::new().padding(Padding {
            left: 1,
            right: 1,
            top: 0,
            bottom: 0,
        }))
        .wrap(Wrap { trim: true })
    }

    fn build_menu_widget(&self) -> List {
        let pages: [String; 4] = [
            String::from("about"),
            String::from("experience"),
            String::from("projects"),
            String::from("leadership"),
        ];

        let menu_items: Vec<ListItem> = (0..pages.len())
            .map(move |index| {
                let item_content = match index == self.selected_page {
                    true => format!("[ {} ]", pages[index]),
                    false => pages[index].to_owned(),
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
                    bottom: 0,
                    right: 2,
                    left: 0,
                }),
        );

        final_list
    }
}
