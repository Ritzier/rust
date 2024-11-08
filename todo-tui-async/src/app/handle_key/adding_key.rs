use ratatui::crossterm::event::KeyCode;

use crate::app::{todo::TodoItem, AppState};

use super::App;

impl App {
    pub fn handle_adding_key(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Esc => self.adding_to_main(),
            KeyCode::Tab => self.toggle_currently_editing(),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Char(char) => self.push_char(char),
            KeyCode::Enter => self.add_todo(),
            _ => {}
        }
    }

    fn adding_to_main(&mut self) {
        self.clear_input();
        self.appstate = AppState::Main
    }

    fn add_todo(&mut self) {
        if !self.todo_input.is_empty() && !self.description_input.is_empty() {
            let todo_item = TodoItem::new(
                false,
                std::mem::take(&mut self.todo_input),
                std::mem::take(&mut self.description_input),
            );
            self.todo_list.todo.push(todo_item);
            self.currently_editing = None
        }
        self.appstate = AppState::Main
    }
}
