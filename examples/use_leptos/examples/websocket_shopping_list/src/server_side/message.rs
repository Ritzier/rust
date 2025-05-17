use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Item;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Connect,
    Disconnect,
    Welcome { list: Vec<Item> },
    Add { id: Uuid, label: String },
    Remove { id: Uuid },
    MarkComplete { id: Uuid, completed: bool },
    Edit { id: Uuid, new_label: String },
}
