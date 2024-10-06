use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::palette::tailwind::{BLUE, SLATE};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, Padding, Paragraph, StatefulWidget, Widget,
    Wrap,
};

use super::{alternate_colors, App};

impl App {
    pub fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Todo List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(Style::new().fg(SLATE.c100).bg(BLUE.c800))
            .bg(SLATE.c950);

        let items: Vec<ListItem> = self
            .todo_list
            .todo
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.todo_list.state)
    }

    pub fn render_selected(&self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.todo_list.state.selected() {
            match self.todo_list.todo[i].completed {
                true => format!("✓ DONE: {}", self.todo_list.todo[i].description),
                false => format!("☐ TODO: {}", self.todo_list.todo[i].description),
            }
        } else {
            "Nothing selected...".to_string()
        };

        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(Style::new().fg(SLATE.c100).bg(BLUE.c800))
            .bg(SLATE.c950)
            .padding(Padding::horizontal(1));

        Paragraph::new(info)
            .block(block)
            .bg(SLATE.c950)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}
