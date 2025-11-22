use crate::pages::page::Page;
use crate::pages::style::{dimmed_link_style, gray_span, line_from_spans, link_span, white_span};
use bincode::{Decode, Encode};
use crossterm::event::KeyCode;
use image::ImageReader;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::canvas::{Canvas, Points},
    widgets::{Block, Padding, Paragraph, Wrap},
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

fn osc52(text: &str) {
    use base64::{Engine as _, engine::general_purpose};

    let encoded = general_purpose::STANDARD.encode(text.as_bytes());
    print!("\x1b]52;c;{}\x07", encoded);
    // Flush to ensure the sequence is sent immediately
    io::stdout().flush().unwrap();
}

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
    show_tooltip: bool,
    tooltip_end_tick: u64,
}

impl<'a> Page for About<'a> {
    fn title(&self) -> &str {
        "about"
    }

    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let [tooltip_area, content_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);

        if self.show_tooltip {
            let tooltip_text = "✔ contact link copied to clipboard";
            let tooltip_paragraph = Paragraph::new(tooltip_text)
                .style(ratatui::style::Style::new().fg(ratatui::style::Color::Green))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(tooltip_paragraph, tooltip_area);
        }

        let line_1 = line_from_spans(vec![
            gray_span("hey! my name is "),
            white_span("kieran llarena"),
        ]);

        let line_2 = line_from_spans(vec![
            gray_span("im currently studying "),
            white_span("computer science "),
            gray_span("at the "),
            white_span("university of michigan-dearborn"),
        ]);

        let line_3 = line_from_spans(vec![
            gray_span("my expected graduation date is "),
            white_span("may 2027"),
        ]);

        let line_4 = line_from_spans(vec![
            gray_span("i thrive best in environments that value "),
            white_span("high velocity "),
            gray_span("and "),
            white_span("strong ownership"),
        ]);

        let line_5 = line_from_spans(vec![
            gray_span("my background is rooted in "),
            white_span("web and mobile fullstack development"),
            gray_span(", all "),
            white_span("self-taught"),
            gray_span(" through research, experimentation, and project work"),
        ]);

        let line_6 = line_from_spans(vec![
            gray_span("im currently exploring "),
            white_span("systems programming"),
            gray_span(", specifically working with "),
            white_span("embedded rust on microcontrollers"),
        ]);

        let mut links: Vec<Line<'_>> = (0..(self.links.len()))
            .map(|index| {
                let current_contact_link = &self.links[index];
                let is_selected = index == self.state;

                if is_selected {
                    if is_focused {
                        Line::from(vec![
                            link_span(current_contact_link.display_text),
                            white_span(" - "),
                            link_span(current_contact_link.link),
                        ])
                    } else {
                        Line::from(vec![
                            Span::styled(current_contact_link.display_text, dimmed_link_style()),
                            white_span(" - "),
                            Span::styled(current_contact_link.link, dimmed_link_style()),
                        ])
                    }
                } else {
                    Line::from(vec![
                        gray_span(current_contact_link.display_text),
                        gray_span(" - "),
                        gray_span(current_contact_link.link),
                    ])
                }
            })
            .collect();

        let mut lines: Vec<Line<'_>> = vec![
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
            Line::from(white_span("contact:")),
        ];

        lines.append(&mut links);

        let paragraph = Paragraph::new(lines)
            .block(Block::new().padding(Padding {
                left: 1,
                right: 2,
                top: 0,
                bottom: 0,
            }))
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, content_area);
    }

    fn render_additional(&self, frame: &mut Frame, area: Rect, _is_focused: bool) {
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
            KeyCode::Up => {
                if self.state > 0 {
                    self.state -= 1;
                }
                self.update_current_link();
            }
            KeyCode::Down => {
                if self.state < self.links.len() - 1 {
                    self.state += 1;
                }
                self.update_current_link();
            }
            KeyCode::Enter => {
                if !self.current_link.is_empty() {
                    osc52(&self.current_link);
                    self.show_tooltip = true;
                    self.tooltip_end_tick = self.tick + 38;
                }
            }
            _ => {}
        }
    }

    fn nav_items(&self) -> Vec<Line<'static>> {
        vec![line_from_spans(vec![white_span(" ↵  "), gray_span("copy")])]
    }

    fn on_tick(&mut self, tick: u64) -> bool {
        self.tick = tick;
        if self.show_tooltip && tick >= self.tooltip_end_tick {
            self.show_tooltip = false;
        }
        true
    }
}

impl<'a> About<'a> {
    fn update_current_link(&mut self) {
        if let Some(selected_link) = self.links.get(self.state) {
            self.current_link = selected_link.link.to_string();
        }
    }

    pub fn new(show_debug_frames: bool) -> Self {
        let links: Vec<ContactLink> = vec![
            ContactLink {
                display_text: "twitter",
                link: "x.com/krayondev",
            },
            ContactLink {
                display_text: "linkedin",
                link: "linkedin.com/in/kllarena07",
            },
            ContactLink {
                display_text: "github",
                link: "github.com/kllarena07",
            },
            ContactLink {
                display_text: "email",
                link: "kieran.llarena@gmail.com",
            },
        ];

        let all_frames = get_all_frames_rgb_vals(show_debug_frames);
        let max_frames = all_frames.len();

        let initial_link = links
            .first()
            .map(|link| link.link.to_string())
            .unwrap_or_default();

        Self {
            state: 0,
            current_link: initial_link,
            links,
            all_frames,
            max_frames,
            tick: 0,
            show_tooltip: false,
            tooltip_end_tick: 0,
        }
    }
}

#[derive(Encode, Decode)]
struct FrameCache {
    frames: Vec<Vec<Vec<[u8; 3]>>>,
}

fn get_all_frames_rgb_vals(show_debug_frames: bool) -> Vec<Vec<Vec<[u8; 3]>>> {
    const CACHE_FILE: &str = "./hikari-dance/frames_cache.bin";

    // Try to load from cache first
    if Path::new(CACHE_FILE).exists() {
        if show_debug_frames {
            println!("Loading frames from cache...");
        }
        if let Ok(cached_frames) = load_frames_from_cache(CACHE_FILE) {
            if show_debug_frames {
                println!(
                    "Successfully loaded {} frames from cache",
                    cached_frames.len()
                );
            }
            return cached_frames;
        }
        if show_debug_frames {
            println!("Cache load failed, recalculating frames...");
        }
    }

    if show_debug_frames {
        println!("Cache not found, processing frames...");
    }

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
    if show_debug_frames {
        println!("Frame loading order (first 10):");
        for (i, path) in frame_files.iter().take(10).enumerate() {
            println!("{}: {}", i, path.file_name().unwrap().to_string_lossy());
        }
    }

    let all_frames: Vec<Vec<Vec<[u8; 3]>>> = frame_files
        .par_iter()
        .filter_map(|frame_path| {
            ImageReader::open(frame_path)
                .ok()?
                .decode()
                .ok()
                .map(|decoded_img| {
                    // Resize to square dimensions
                    let resized_img =
                        decoded_img.resize(112, 112, image::imageops::FilterType::Lanczos3);
                    let rgb_img = resized_img.to_rgb8();
                    let (width, height) = rgb_img.dimensions();

                    // Create 2D array to store RGB values for this frame
                    let pixel_rgb_val_map: Vec<Vec<[u8; 3]>> = (0..height)
                        .into_par_iter()
                        .map(|y| {
                            (0..width)
                                .map(|x| {
                                    let pixel = rgb_img.get_pixel(x, y);
                                    [pixel[0], pixel[1], pixel[2]]
                                })
                                .collect()
                        })
                        .collect();

                    pixel_rgb_val_map
                })
        })
        .collect();

    // Save to cache for future use
    if let Err(e) = save_frames_to_cache(&all_frames, CACHE_FILE) {
        eprintln!("Warning: Failed to save frames to cache: {}", e);
    } else {
        if show_debug_frames {
            println!("Successfully cached {} frames", all_frames.len());
        }
    }

    all_frames
}

fn save_frames_to_cache(
    frames: &[Vec<Vec<[u8; 3]>>],
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache = FrameCache {
        frames: frames.to_vec(),
    };
    let config = bincode::config::standard();
    let encoded = bincode::encode_to_vec(&cache, config)?;
    let mut file = fs::File::create(path)?;
    file.write_all(&encoded)?;
    Ok(())
}

fn load_frames_from_cache(
    path: &str,
) -> Result<Vec<Vec<Vec<[u8; 3]>>>, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let config = bincode::config::standard();
    let (cache, _): (FrameCache, _) = bincode::decode_from_slice(&buffer, config)?;
    Ok(cache.frames)
}
