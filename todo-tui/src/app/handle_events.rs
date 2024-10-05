use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use super::{todo::TodoItem, App, AppState, CurrentlyEditing};

impl App {
    pub fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_millis(250);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match self.appstate {
                        AppState::Main => match key.code {
                            // AppState::Main => AppState::Exiting
                            KeyCode::Char('q') => self.appstate = AppState::Exiting,
                            // AppState::Editing => AppState::Main
                            KeyCode::Char('e') => {
                                self.appstate = AppState::Editing;
                                self.currently_editing = Some(CurrentlyEditing::Todo);
                            }
                            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                            KeyCode::Char('k') | KeyCode::Up => self.select_prev(),
                            KeyCode::Char(' ') | KeyCode::Enter => self.toggle_todo(),
                            KeyCode::Char('d') | KeyCode::Backspace => {
                                if let Some(i) = self.todo_list.state.selected() {
                                    self.currently_usize = Some(i);
                                    self.appstate = AppState::Deleting;
                                }
                            }
                            _ => {}
                        },

                        AppState::Editing => match key.code {
                            // Exit from editing to main
                            KeyCode::Esc => {
                                self.appstate = AppState::Main;
                                self.currently_editing = None
                            }
                            // Toggle key value editing
                            KeyCode::Tab => self.toggle_editing(),
                            KeyCode::Char(v) => {
                                if let Some(currently_editing) = &self.currently_editing {
                                    match currently_editing {
                                        CurrentlyEditing::Todo => self.todo_input.push(v),
                                        CurrentlyEditing::Description => {
                                            self.description_input.push(v)
                                        }
                                    }
                                }
                            }
                            // Delete current string
                            KeyCode::Backspace => {
                                if let Some(currently_editing) = &self.currently_editing {
                                    match currently_editing {
                                        CurrentlyEditing::Todo => {
                                            self.todo_input.pop();
                                        }
                                        CurrentlyEditing::Description => {
                                            self.description_input.pop();
                                        }
                                    }
                                }
                            }
                            // Push current key,value string -> TodoItem
                            KeyCode::Enter => {
                                if let Some(currently_editing) = &self.currently_editing {
                                    match currently_editing {
                                        CurrentlyEditing::Todo => {
                                            self.currently_editing =
                                                Some(CurrentlyEditing::Description)
                                        }
                                        CurrentlyEditing::Description => {
                                            let new_todo = TodoItem::new(
                                                false,
                                                self.todo_input.clone(),
                                                self.description_input.clone(),
                                            );
                                            self.todo_list.todo.push(new_todo);
                                            self.todo_input = String::new();
                                            self.description_input = String::new();
                                            self.currently_editing = None;
                                            self.appstate = AppState::Main;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },

                        AppState::Deleting => match key.code {
                            KeyCode::Char('y') => {
                                self.delete_current_todo();
                                self.editing_to_main();
                            }
                            KeyCode::Char('n') => self.editing_to_main(),
                            _ => {}
                        },

                        AppState::Exiting => match key.code {
                            KeyCode::Char('y') => self.quit = true,
                            KeyCode::Char('n') => self.appstate = AppState::Main,
                            _ => {}
                        },
                    }
                }
            }
        }
        Ok(())
    }
}
