use crossterm::event::KeyCode;
use image::ImageReader;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Style, Stylize},
    symbols,
    text::Span,
    widgets::{Block, Borders, List, ListItem, Padding},
};
use std::fs;
use std::{io, sync::mpsc, thread, time::Duration};

mod pages;
use pages::{about::About, page::Page};

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

    let pages: Vec<Box<dyn Page>> = vec![Box::new(About::new())];

    let mut app = App {
        running: true,
        selected_page: 0,
        frame_number: 0,
        pages,
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
    frame_number: usize,
    max_frames: usize,
    pages: Vec<Box<dyn Page>>,
    all_frames: Vec<Vec<Vec<[u8; 3]>>>,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while self.running {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Counter(count) => self.frame_number = count,
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
        let current_frame_index = self.frame_number % self.max_frames;
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
        // frame.render_widget(
        //     Block::new()
        //         .fg(Color::Blue)
        //         .title("Right")
        //         .borders(Borders::ALL),
        //     canvas_area,
        // );

        let menu_widget = self.build_menu_widget();
        frame.render_widget(menu_widget, menu_area);

        if let Some(current_page) = self.pages.get(self.selected_page) {
            current_page.render(frame, center_area);
            current_page.render_additional(frame, canvas_area);
        }

        // match self.selected_page {
        //     0 => frame.render_widget(canvas, canvas_area),
        //     1 => {
        //         let description = self.experience_page.build_description();
        //         frame.render_widget(description, canvas_area)
        //     }
        //     _ => {}
        // };
    }

    fn previous_page(&mut self) {
        if self.selected_page > 0 {
            self.selected_page -= 1;
        }
    }

    fn next_page(&mut self) {
        if self.selected_page + 1 < self.pages.len() {
            self.selected_page += 1;
        }
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => {
                self.running = false;
            }
            KeyCode::Up => self.previous_page(),
            KeyCode::Down => self.next_page(),
            _ => {
                if let Some(current_page) = self.pages.get_mut(self.selected_page) {
                    let _ = current_page.keyboard_event_handler(key_event.code);
                }
            }
        }

        Ok(())
    }

    fn build_menu_widget(&self) -> List<'_> {
        let menu_items: Vec<ListItem> = (0..self.pages.len())
            .map(move |index| {
                let title = self.pages[index].title();
                let item_content = if index == self.selected_page {
                    format!("[ {} ]", title)
                } else {
                    title.to_string()
                };

                let span = if index == self.selected_page {
                    Span::styled(item_content, Style::default().fg(Color::Rgb(255, 255, 255)))
                } else {
                    Span::styled(item_content, Style::default().fg(Color::Rgb(147, 147, 147)))
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
