use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Color},
    text::Line,
    widgets::{ListItem, Widget},
};

use super::{todo::TodoItem, App};

mod content;
mod navbar;
mod popup_adding;
mod popup_deleting;
mod popup_editing;
mod popup_exiting;
mod title;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [title_bar, content_area, nav_bar] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(area);

        let [vertical1, vertical2] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(content_area);

        App::render_title(title_bar, buf);
        self.render_nav(nav_bar, buf);
        self.render_list(vertical1, buf);
        self.render_selected(vertical2, buf);

        self.render_popup_adding(area, buf);
        self.render_popup_editing(area, buf);
        self.render_popup_exiting(area, buf);
        self.render_popup_deleting(area, buf);
    }
}

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
