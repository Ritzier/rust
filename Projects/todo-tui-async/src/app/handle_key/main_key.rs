use crossterm::event::KeyCode;

use crate::{app::AppState, App};

impl App {
    pub fn handle_main_key(&mut self, key: KeyCode) {
        match key {
            // AppState::Main => AppState::Exiting
            KeyCode::Char('q') | KeyCode::Esc => self.main_to_exiting(),
            // AppState::Main => AppState::Adding
            KeyCode::Char('a') => self.main_to_adding(),
            // AppState::Main => AppState::Editing
            KeyCode::Char('e') => self.main_to_editing(),
            // AppState::Main => AppState::Deleting
            KeyCode::Char('d') => self.main_to_deleting(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_prev(),
            KeyCode::Char(' ') | KeyCode::Enter => self.toggle_todo(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }

    fn select_prev(&mut self) {
        self.todo_list.state.select_previous();
    }

    fn get_selected(&mut self) -> Option<usize> {
        self.todo_list.state.selected()
    }

    fn toggle_todo(&mut self) {
        if let Some(i) = self.get_selected() {
            self.todo_list.todo[i].completed = !self.todo_list.todo[i].completed
        }
    }

    fn main_to_exiting(&mut self) {
        self.appstate = AppState::Exiting
    }

    fn main_to_adding(&mut self) {
        self.appstate = AppState::Adding;
    }

    fn main_to_editing(&mut self) {
        if let Some(i) = self.get_selected() {
            self.todo_input = self.todo_list.todo[i].todo.clone();
            self.description_input = self.todo_list.todo[i].description.clone();
            self.appstate = AppState::Editing(i);
        }
    }

    fn main_to_deleting(&mut self) {
        if let Some(i) = self.get_selected() {
            self.appstate = AppState::Deleting(i)
        }
    }
}
