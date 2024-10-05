use color_eyre::Result;
use ratatui::DefaultTerminal;

use todo::TodoList;

mod handle_events;
mod result;
mod theme;
mod todo;
mod ui;

pub struct App {
    todo_input: String,
    description_input: String,
    appstate: AppState,
    todo_list: TodoList,
    file: String,
    quit: bool,
    currently_editing: Option<CurrentlyEditing>,
    currently_usize: Option<usize>,
}

#[derive(PartialEq, Eq)]
enum AppState {
    Main,
    Editing,
    Deleting,
    Exiting,
}

enum CurrentlyEditing {
    Todo,
    Description,
}

impl App {
    pub fn new(file: String) -> std::io::Result<Self> {
        Ok(Self {
            todo_input: String::new(),
            description_input: String::new(),
            appstate: AppState::Main,
            todo_list: TodoList::new(&file)?,
            file,
            currently_editing: None,
            quit: false,
            currently_usize: None,
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.quit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }

        self.todo_list.to_json(&self.file)?;

        Ok(())
    }

    fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }

    fn select_prev(&mut self) {
        self.todo_list.state.select_previous();
    }

    fn toggle_todo(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.todo[i].completed = !self.todo_list.todo[i].completed
        }
    }

    fn toggle_editing(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Todo => {
                    self.currently_editing = Some(CurrentlyEditing::Description)
                }
                CurrentlyEditing::Description => {
                    self.currently_editing = Some(CurrentlyEditing::Todo)
                }
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Todo)
        }
    }

    fn editing_to_main(&mut self) {
        self.currently_usize = None;
        self.appstate = AppState::Main;
    }

    fn delete_current_todo(&mut self) {
        if let Some(i) = self.currently_usize {
            self.todo_list.todo.remove(i);
        }
    }
}
