use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use tokio::fs::File;

use crate::Result;

pub struct TodoList {
    pub todo: Vec<TodoItem>,
    pub state: ListState,
}

#[derive(Deserialize, Serialize)]
pub struct TodoItem {
    pub completed: bool,
    pub todo: String,
    pub description: String,
}

impl TodoList {
    pub async fn new(file: &str) -> Result<Self> {
        let todo = Self::from_json(file).await.unwrap_or_default();
        Ok(Self {
            todo,
            state: ListState::default(),
        })
    }

    async fn from_json(file: &str) -> Result<Vec<TodoItem>> {
        let file = File::open(file).await?;
        let reader = file.into_std().await;
        let todo_list: Vec<TodoItem> = serde_json::from_reader(reader)?;
        Ok(todo_list)
    }

    pub async fn to_json(&self, file: &str) -> Result<()> {
        let file = File::create(file).await?;
        let reader = file.into_std().await;
        serde_json::to_writer(reader, &self.todo)?;
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
