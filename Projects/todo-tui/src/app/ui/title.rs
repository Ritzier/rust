use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use super::App;

impl App {
    pub fn render_title(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Todo Application").render(area, buf);
    }
}
