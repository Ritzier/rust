use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app::App;

impl App {
    pub fn render_title(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Downloader").render(area, buf);
    }
}
