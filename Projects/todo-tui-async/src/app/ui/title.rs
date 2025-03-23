use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use super::App;

impl App {
    pub fn render_title(area: Rect, frame: &mut Frame) {
        let paragraph = Paragraph::new("Todo Application");
        frame.render_widget(paragraph, area);
    }
}
