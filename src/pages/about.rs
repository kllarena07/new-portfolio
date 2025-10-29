use crate::pages::page::Page;
use crate::pages::style::{accent_underlined_style, gray_span, gray_style, white_span};
use crossterm::event::KeyCode;
use image::ImageReader;
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::canvas::{Canvas, Points},
    widgets::{Block, Padding, Paragraph, Wrap},
};
use std::fs;

#[derive(Clone)]
pub struct ContactLink<'a> {
    pub display_text: &'a str,
    pub link: &'a str,
}

pub struct About<'a> {
    state: usize,
    current_link: String,
    links: Vec<ContactLink<'a>>,
    all_frames: Vec<Vec<Vec<[u8; 3]>>>,
    max_frames: usize,
    tick: u64,
}

impl<'a> Page for About<'a> {
    fn title(&self) -> &str {
        "about"
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let line_1 = Line::from(vec![
            gray_span("hey! my name is "),
            white_span("kieran llarena"),
        ]);

        let line_2 = Line::from(vec![
            gray_span("im currently studying "),
            white_span("computer science "),
            gray_span("at the "),
            white_span("university of michigan-dearborn"),
        ]);

        let line_3 = Line::from(vec![
            gray_span("my expected graduation date is "),
            white_span("may 2027"),
        ]);

        let line_4 = Line::from(vec![
            gray_span("i thrive best in environments that value "),
            white_span("high velocity "),
            gray_span("and "),
            white_span("strong ownership"),
        ]);

        let line_5 = Line::from(vec![
            gray_span("my background is rooted in "),
            white_span("web and mobile fullstack development"),
        ]);

        let line_6 = Line::from(vec![
            gray_span("im currently exploring "),
            white_span("systems programming"),
            gray_span(", specifically working with "),
            white_span("embedded Rust on microcontrollers"),
        ]);

        let line_items: Vec<Span> = (0..(self.links.len() * 2) - 1)
            .map(move |index| {
                if (index + 1) % 2 == 0 {
                    return gray_span(" - ");
                }

                let style_config = match index / 2 == self.state {
                    true => accent_underlined_style(),
                    false => gray_style(),
                };

                let display_text = self.links[index / 2].display_text.to_owned();

                Span::styled(display_text, style_config)
            })
            .collect();

        let links_line = Line::from(line_items);

        let paragraph = Paragraph::new(vec![
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
        .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    fn render_additional(&self, frame: &mut Frame, area: Rect) {
        if self.max_frames == 0 || self.all_frames.is_empty() {
            return;
        }

        let idx = (self.tick as usize) % self.max_frames;
        let current_frame = &self.all_frames[idx];
        let frame_height = current_frame.len() as f64;
        if frame_height == 0.0 {
            return;
        }
        let frame_width = current_frame[0].len() as f64;
        let y_max = frame_height * 2.0; // HalfBlock doubles vertical resolution

        let canvas = Canvas::default()
            .marker(ratatui::symbols::Marker::HalfBlock)
            .x_bounds([0.0, frame_width])
            .y_bounds([0.0, y_max])
            .paint(|ctx| {
                for (y, row) in current_frame.iter().enumerate() {
                    let y_top = y_max - (y as f64 * 2.0);
                    for (x, pixel) in row.iter().enumerate() {
                        let x_f = x as f64;
                        ctx.draw(&Points {
                            coords: &[(x_f, y_top)],
                            color: ratatui::style::Color::Rgb(pixel[0], pixel[1], pixel[2]),
                        });
                    }
                }
            });

        frame.render_widget(canvas, area);
    }

    fn keyboard_event_handler(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Left => {
                self.previous_link();
            }
            KeyCode::Right => {
                self.next_link();
            }
            KeyCode::Enter => {
                open::that(&self.current_link).unwrap();
            }
            _ => {}
        }
    }

    fn on_tick(&mut self, tick: u64) -> bool {
        self.tick = tick;
        true
    }
}

impl<'a> About<'a> {
    pub fn new() -> Self {
        let links: Vec<ContactLink> = vec![
            ContactLink {
                display_text: "twitter",
                link: "https://x.com/krayondev",
            },
            ContactLink {
                display_text: "linkedin",
                link: "https://www.linkedin.com/in/kllarena07/",
            },
            ContactLink {
                display_text: "github",
                link: "https://github.com/kllarena07",
            },
            ContactLink {
                display_text: "email",
                link: "mailto:kieran.llarena@gmail.com",
            },
        ];

        let all_frames = get_all_frames_rgb_vals();
        let max_frames = all_frames.len();

        Self {
            state: 0,
            current_link: String::from(links[0].link),
            links,
            all_frames,
            max_frames,
            tick: 0,
        }
    }

    fn previous_link(&mut self) {
        if self.state > 0 {
            self.state -= 1;
            self.change_current_link();
        }
    }

    fn next_link(&mut self) {
        if self.state + 1 < self.links.len() {
            self.state += 1;
            self.change_current_link();
        }
    }

    fn change_current_link(&mut self) {
        self.current_link = String::from(self.links[self.state].link);
    }
}

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
