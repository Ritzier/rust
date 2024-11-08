use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::{AppState, CurrentlyEditing};

use super::{percentage_centered_area, App};

impl App {
    pub fn render_popup_editing(&self, area: Rect, frame: &mut Frame) {
        if let AppState::Editing(_) = self.appstate {
            let popup_block = Block::default()
                .title("Enter a new key-value pair")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let area = percentage_centered_area(60, 25, area);
            frame.render_widget(Clear, area);
            frame.render_widget(popup_block, area);

            let popup_chunks =
                Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .margin(1)
                    .split(area);

            let mut key_block = Block::default().title("Key").borders(Borders::ALL);
            let mut value_block = Block::default().title("Value").borders(Borders::ALL);

            let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

            if let Some(currently_editing) = &self.currently_editing {
                match currently_editing {
                    CurrentlyEditing::Todo => key_block = key_block.style(active_style),
                    CurrentlyEditing::Description => value_block = value_block.style(active_style),
                }
            }

            let key_text = Paragraph::new(self.todo_input.clone()).block(key_block);
            frame.render_widget(key_text, popup_chunks[0]);
            let value_text = Paragraph::new(self.description_input.clone()).block(value_block);
            frame.render_widget(value_text, popup_chunks[1]);
        }
    }
}
