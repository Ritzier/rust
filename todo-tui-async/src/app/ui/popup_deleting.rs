use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::AppState;

use super::{centered_area, App};

impl App {
    pub fn render_popup_deleting(&self, area: Rect, frame: &mut Frame) {
        if let AppState::Deleting(_) = self.appstate {
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
            frame.render_widget(Clear, area);
            frame.render_widget(delete_paragraph, area);
        }
    }
}
