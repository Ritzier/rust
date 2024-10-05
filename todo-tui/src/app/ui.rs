use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, HighlightSpacing, List, ListItem, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
};

use super::{todo::TodoItem, App, AppState, CurrentlyEditing};

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
        App::render_nav(nav_bar, buf);
        self.render_list(vertical1, buf);
        self.render_selected(vertical2, buf);

        self.render_popup_editing(area, buf);
        self.render_popup_exiting(area, buf);
        self.render_popup_deleting(area, buf);
    }
}

impl App {
    fn render_title(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Todo Application").render(area, buf);
    }

    // TODO: current mode | key hint
    fn render_nav(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Nav").render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
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

    fn render_selected(&self, area: Rect, buf: &mut Buffer) {
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

    fn render_popup_editing(&self, area: Rect, buf: &mut Buffer) {
        if let Some(editing) = &self.currently_editing {
            let popup_block = Block::default()
                .title("Enter a new key-value pair")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let area = percentage_centered_area(60, 25, area);
            Clear.render(area, buf);
            popup_block.render(area, buf);

            let popup_chunks =
                Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .margin(1)
                    .split(area);

            let mut key_block = Block::default().title("Key").borders(Borders::ALL);
            let mut value_block = Block::default().title("Value").borders(Borders::ALL);

            let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

            match &editing {
                CurrentlyEditing::Todo => key_block = key_block.style(active_style),
                CurrentlyEditing::Description => value_block = value_block.style(active_style),
            }

            let key_text = Paragraph::new(self.todo_input.clone()).block(key_block);
            key_text.render(popup_chunks[0], buf);
            let value_text = Paragraph::new(self.description_input.clone()).block(value_block);
            value_text.render(popup_chunks[1], buf);
        }
    }

    fn render_popup_exiting(&self, area: Rect, buf: &mut Buffer) {
        if self.appstate == AppState::Exiting {
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::DarkGray));

            let exit_text = vec![Line::from(vec![
                Span::styled("Are you want to quit?", Style::default().fg(Color::Red)),
                Span::styled(" (y/n)", Style::default().fg(Color::DarkGray)),
            ])];

            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .centered()
                .wrap(Wrap { trim: false });

            let area = centered_area(3, 40, area);
            Clear.render(area, buf);
            exit_paragraph.render(area, buf);
        }
    }

    fn render_popup_deleting(&self, area: Rect, buf: &mut Buffer) {
        if self.appstate == AppState::Deleting {
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::DarkGray));

            let delete_text = vec![Line::from(vec![
                Span::styled("Delete current todo?", Style::default().fg(Color::Red)),
                Span::styled(" (y/n)", Style::default().fg(Color::DarkGray)),
            ])];

            let delete_paragraph = Paragraph::new(delete_text)
                .block(popup_block)
                .centered()
                .wrap(Wrap { trim: false });

            let area = centered_area(3, 40, area);
            Clear.render(area, buf);
            delete_paragraph.render(area, buf);
        }
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
            false => Line::styled(format!(" ☐ {}", value.info), SLATE.c200),
            true => Line::styled(format!(" ✓ {}", value.info), SLATE.c500),
        };
        ListItem::new(line)
    }
}
