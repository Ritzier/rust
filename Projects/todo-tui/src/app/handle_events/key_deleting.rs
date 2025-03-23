use ratatui::crossterm::event::KeyCode;

use crate::app::AppState;

use super::App;

impl App {
    pub fn deleting_key_events(&mut self, keycode: KeyCode, i: usize) {
        match keycode {
            KeyCode::Char('y') => self.delete_current_todo(i),
            KeyCode::Char('n') => self.deleting_to_main(),
            _ => {}
        }
    }

    // After delete a todo item, return back to AppState::Main
    fn delete_current_todo(&mut self, i: usize) {
        //if let Some(i) = self.currently_usize {
        //    self.todo_list.todo.remove(i);
        //}

        // INFO:
        // Manual get once current state
        // After self.currently_usize == Some(i), and after it none selected,
        // it wont fallback to None
        self.todo_list.todo.remove(i);
        self.appstate = AppState::Main
    }

    fn deleting_to_main(&mut self) {
        self.appstate = AppState::Main
    }
}
