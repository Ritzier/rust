use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::app::AppState;

use super::{centered_area, App};

impl App {
    pub fn render_popup_exiting(&self, area: Rect, buf: &mut Buffer) {
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
}
