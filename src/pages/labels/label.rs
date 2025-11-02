use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::{Block, Padding, Paragraph},
};

pub struct ColoredLabel {
    pub title: &'static str,
    pub fg: Color,
    pub bg: Color,
}

impl ColoredLabel {
    pub fn new(title: &'static str, fg: Color, bg: Color) -> Self {
        Self { title, fg, bg }
    }

    pub fn width(&self) -> u16 {
        self.title.len() as u16
    }

    pub fn to_paragraph(&self) -> Paragraph<'static> {
        let title_text = Text::from(self.title);
        let style_config = Style::default().fg(self.fg).bg(self.bg);
        let block_config = Block::new().padding(Padding {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        });

        Paragraph::new(title_text)
            .style(style_config)
            .block(block_config)
    }
}
