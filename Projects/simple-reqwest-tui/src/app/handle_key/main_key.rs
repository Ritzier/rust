use crossterm::event::KeyCode;

use crate::app::{App, AppState};

impl App {
    pub fn handle_main_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('a') => self.appstate = AppState::Input,
            KeyCode::Char('j') | KeyCode::Down => self.list_state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.list_state.select_previous(),
            _ => {}
        }
    }
}
