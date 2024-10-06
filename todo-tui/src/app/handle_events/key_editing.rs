use ratatui::crossterm::event::KeyCode;

use crate::app::{todo::TodoItem, AppState};

use super::App;

impl App {
    pub fn editing_key_events(&mut self, keycode: KeyCode, i: usize) {
        match keycode {
            KeyCode::Esc => self.editing_to_main(),
            KeyCode::Tab => self.toggle_currently_editing(),
            KeyCode::Backspace => self.backspace_char(),
            KeyCode::Char(char) => self.push_char(char),
            KeyCode::Enter => self.modify_todo(i),
            _ => {}
        }
    }

    fn editing_to_main(&mut self) {
        self.clear_input();
        self.appstate = AppState::Main;
    }

    fn modify_todo(&mut self, i: usize) {
        if !self.todo_input.is_empty() && !self.description_input.is_empty() {
            let todo_item = TodoItem::new(
                false,
                std::mem::take(&mut self.todo_input),
                std::mem::take(&mut self.description_input),
            );
            self.todo_list.todo[i] = todo_item;
            self.currently_editing = None;
        }
        self.editing_to_main();
    }
}
