use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::app::{App, AppState};

use super::center_area;

impl App {
    pub fn render_input_popup(&mut self, area: Rect, buf: &mut Buffer) {
        if self.appstate != AppState::Input {
            return;
        }

        let text_width: u16 = if self.url_input.len() < 25 {
            25
        } else {
            self.url_input.len() as u16 + 3
        };

        let popup_area = center_area(area, Constraint::Length(text_width), Constraint::Length(3));

        let paragraph =
            Paragraph::new(self.url_input.clone()).block(Block::bordered().title("Input"));

        Clear.render(popup_area, buf);
        paragraph.render(popup_area, buf);
    }
}
