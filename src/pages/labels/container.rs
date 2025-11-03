use ratatui::{Frame, layout::Rect};

use super::label::ColoredLabel;

pub struct LabelContainer<'a> {
    labels: &'a [ColoredLabel],
}

impl<'a> LabelContainer<'a> {
    pub fn new(labels: &'a [ColoredLabel]) -> Self {
        Self { labels }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.labels.is_empty() {
            return;
        }

        let mut current_x = area.x;
        let mut current_y = area.y;

        for label in self.labels {
            let label_width = label.width();

            // Check if we need to wrap to the next line
            if current_x + label_width > area.x + area.width {
                current_y += 2; // +2 for 1px vertical spacing
                current_x = area.x;

                // Stop if we've run out of vertical space
                if current_y >= area.y + area.height {
                    break;
                }
            }

            // Render the label at the current position
            let label_area = Rect {
                x: current_x,
                y: current_y,
                width: label_width,
                height: 1,
            };

            frame.render_widget(label.to_paragraph(), label_area);

            // Move to the next position (label width + 2 for spacing)
            current_x += label_width + 2;
        }
    }
}
