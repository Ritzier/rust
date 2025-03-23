use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Color},
    text::Line,
    widgets::ListItem,
    Frame,
};

use super::{todo::TodoItem, App};

mod content;
mod navbar;
mod popup_adding;
mod popup_deleting;
mod popup_editing;
mod popup_exiting;
mod title;

//impl Widget for &mut App {
impl App {
    pub fn render(&mut self, area: Rect, frame: &mut Frame) {
        let [title_bar, content_area, nav_bar] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(area);

        let [vertical1, vertical2] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(content_area);

        App::render_title(title_bar, frame);
        self.render_nav(nav_bar, frame);
        self.render_list(vertical1, frame);
        self.render_selected(vertical2, frame);

        self.render_popup_adding(area, frame);
        self.render_popup_editing(area, frame);
        self.render_popup_exiting(area, frame);
        self.render_popup_deleting(area, frame);
    }
}
//}

fn percentage_centered_area(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_area = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(area);

    Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(popup_area[1])[1]
}

fn centered_area(height: u16, width: u16, area: Rect) -> Rect {
    let popup_area = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .split(area);

    Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(height),
        Constraint::Fill(1),
    ])
    .split(popup_area[1])[1]
}

fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        SLATE.c950
    } else {
        SLATE.c900
    }
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        let line = match value.completed {
            false => Line::styled(format!(" ☐ {}", value.todo), SLATE.c200),
            true => Line::styled(format!(" ✓ {}", value.todo), SLATE.c500),
        };
        ListItem::new(line)
    }
}
