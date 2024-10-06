use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use super::App;

impl App {
    // TODO: current mode | key hint
    pub fn render_nav(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("nav").render(area, buf);
    }
}
