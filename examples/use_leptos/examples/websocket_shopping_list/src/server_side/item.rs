use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Store, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub label: String,
    pub completed: bool,
}
