use ratatui::crossterm::event::KeyCode;

use crate::app::AppState;

use super::App;

impl App {
    pub fn handle_exiting_key(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Char('y') => self.exit(),
            KeyCode::Char('n') => self.exiting_to_main(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.quit = true;
    }

    fn exiting_to_main(&mut self) {
        self.appstate = AppState::Main;
    }
}
