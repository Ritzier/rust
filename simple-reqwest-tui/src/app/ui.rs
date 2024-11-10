use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Widget,
};

use super::App;

mod content;
mod input;
mod title;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [title_bar, content_bar, _nav_bar] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(area);

        App::render_title(title_bar, buf);
        self.render_list(content_bar, buf);

        // Popup
        self.render_input_popup(area, buf);
    }
}

fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
