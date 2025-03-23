use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::{AppState, CurrentlyEditing};

use super::App;

impl App {
    // TODO: current mode | key hint
    pub fn render_nav(&self, area: Rect, buf: &mut Buffer) {
        let current_navigation_text = vec![
            match self.appstate {
                AppState::Main => Span::styled("NORMAL", Style::default().fg(Color::Blue)),
                AppState::Adding => Span::styled("ADD", Style::default().fg(Color::Green)),
                AppState::Editing(_) => Span::styled("EDIT", Style::default().fg(Color::Yellow)),
                AppState::Deleting(_) => {
                    Span::styled("DELETE", Style::default().fg(Color::LightRed))
                }
                AppState::Exiting => Span::styled("EXIT", Style::default().fg(Color::Red)),
            }
            .to_owned(),
            Span::styled(" | ", Style::default().fg(Color::White)),
            {
                if let Some(editing) = &self.currently_editing {
                    match editing {
                        CurrentlyEditing::Todo => {
                            Span::styled("Editing Todo", Style::default().fg(Color::Green))
                        }
                        CurrentlyEditing::Description => Span::styled(
                            "Editing Description",
                            Style::default().fg(Color::LightGreen),
                        ),
                    }
                } else {
                    Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
                }
            },
        ];

        let mode_footer = Paragraph::new(Line::from(current_navigation_text))
            .block(Block::default().borders(Borders::ALL));

        mode_footer.render(area, buf);
    }
}
