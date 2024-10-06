use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyEventKind};

use super::{App, AppState, CurrentlyEditing};

mod key_adding;
mod key_deleting;
mod key_editing;
mod key_exiting;
mod key_main;

impl App {
    pub fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_millis(250);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match self.appstate {
                        AppState::Main => self.main_key_event(key.code),
                        AppState::Adding => self.adding_key_events(key.code),
                        AppState::Deleting(i) => self.deleting_key_events(key.code, i),
                        AppState::Exiting => self.exiting_key_events(key.code),
                        AppState::Editing(i) => self.editing_key_events(key.code, i),
                    }
                }
            }
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
            Some(CurrentlyEditing::Description) => {
                self.currently_editing = Some(CurrentlyEditing::Todo)
            }
            None => self.currently_editing = Some(CurrentlyEditing::Todo),
        }
    }

    fn push_char(&mut self, character: char) {
        match self.currently_editing {
            Some(CurrentlyEditing::Todo) => self.todo_input.push(character),
            Some(CurrentlyEditing::Description) => self.description_input.push(character),
            None => {
                self.currently_editing = Some(CurrentlyEditing::Todo);
                self.todo_input.push(character)
            }
        }
    }

    fn backspace_char(&mut self) {
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
