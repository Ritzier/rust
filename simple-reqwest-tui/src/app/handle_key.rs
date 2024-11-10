use crossterm::event::KeyCode;

use crate::{App, Result};

use super::AppState;

mod main_key;
mod popup_key;

impl App {
    pub async fn handle_key_event(&mut self, keycode: KeyCode) -> Result<()> {
        match self.appstate {
            AppState::Main => self.handle_main_key(keycode),
            AppState::Input => self.handle_popup_key(keycode).await?,
        }
        Ok(())
    }
}
