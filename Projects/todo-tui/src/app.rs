use color_eyre::Result;
use ratatui::DefaultTerminal;

use todo::TodoList;

mod handle_events;
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
}

#[derive(PartialEq, Eq)]
enum AppState {
    Main,
    Adding,
    Editing(usize),
    Deleting(usize),
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
}
