use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;

pub struct TodoList {
    pub state: ListState,
    pub todo: Vec<TodoItem>,
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub completed: bool,
    pub todo: String,
    pub description: String,
}

impl TodoList {
    pub fn new(file: &str) -> io::Result<Self> {
        let todo = Self::from_json(file.to_string()).unwrap_or_default();
        Ok(Self {
            state: ListState::default(),
            todo,
        })
    }

    fn from_json(file: String) -> io::Result<Vec<TodoItem>> {
        let file = File::open(file)?;
        let todo_list: Vec<TodoItem> = serde_json::from_reader(file)?;
        Ok(todo_list)
    }

    pub fn to_json(&self, file: &String) -> io::Result<()> {
        let file = File::create(file)?;
        serde_json::to_writer(file, &self.todo)?;
        Ok(())
    }
}

impl TodoItem {
    pub fn new(completed: bool, info: String, description: String) -> Self {
        Self {
            completed,
            todo: info,
            description,
        }
    }
}
