use crossterm::event::KeyCode;

mod adding_key;
mod deleting_key;
mod editing_key;
mod exiting_key;
mod main_key;

use super::{App, AppState, CurrentlyEditing, Result};

impl App {
    pub fn handle_key(&mut self, key: KeyCode) -> Result<()> {
        match self.appstate {
            AppState::Adding => self.handle_adding_key(key),
            AppState::Deleting(i) => self.handle_deleting_key(key, i),
            AppState::Editing(i) => self.handle_editing_key(key, i),
            AppState::Exiting => self.handle_exiting_key(key),
            AppState::Main => self.handle_main_key(key),
        }
        Ok(())
    }

    fn clear_input(&mut self) {
        self.todo_input.clear();
        self.description_input.clear();
        self.currently_editing = None;
    }

    fn toggle_currently_editing(&mut self) {
        match self.currently_editing {
            Some(CurrentlyEditing::Todo) => {
                self.currently_editing = Some(CurrentlyEditing::Description)
            }
            Some(CurrentlyEditing::Description) | None => {
                self.currently_editing = Some(CurrentlyEditing::Todo)
            }
        }
    }

    fn push_char(&mut self, char: char) {
        match self.currently_editing {
            Some(CurrentlyEditing::Todo) => self.todo_input.push(char),
            Some(CurrentlyEditing::Description) => self.description_input.push(char),
            None => {
                self.currently_editing = Some(CurrentlyEditing::Todo);
                self.todo_input.push(char)
            }
        }
    }

    fn delete_char(&mut self) {
        match self.currently_editing {
            Some(CurrentlyEditing::Todo) => {
                self.todo_input.pop();
            }
            Some(CurrentlyEditing::Description) => {
                self.description_input.pop();
            }
            None => {}
        }
    }
}
