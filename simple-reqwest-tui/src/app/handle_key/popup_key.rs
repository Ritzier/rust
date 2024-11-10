use crossterm::event::KeyCode;

use crate::{
    app::{App, AppState},
    Result,
};

impl App {
    pub async fn handle_popup_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Esc => {
                if !self.url_input.is_empty() {
                    self.url_input.clear()
                }
                self.appstate = AppState::Main;
            }
            KeyCode::Char(char) => self.url_input.push(char),
            KeyCode::Backspace => {
                self.url_input.pop();
            }
            KeyCode::Enter => {
                let mut url = String::new();
                std::mem::swap(&mut url, &mut self.url_input);
                self.get_url(url).await?;
                self.appstate = AppState::Main;
            }
            _ => {}
        }
        Ok(())
    }
}
