mod client;
mod item;
mod message;
mod websocket;

pub use client::*;
pub use item::*;
pub use message::*;
pub use websocket::websocket_message;

use leptos::prelude::{ReadUntracked, Write};
use reactive_stores::{ArcStore, Field, Store, StoreFieldIterator};
use uuid::Uuid;

pub type MessageWithUser = (Uuid, Message);

#[derive(Debug, Default, Clone, Store, PartialEq, Eq)]
pub struct ShoppingList {
    #[store(key: Uuid=|item| item.id)]
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Copy)]
pub struct State(pub Store<ShoppingList>);

impl From<ArcStore<ShoppingList>> for State {
    fn from(value: ArcStore<ShoppingList>) -> Self {
        State(value.into())
    }
}

impl State {
    /// Applies on update to the local store
    pub fn apply_local_update(&self, message: Message) {
        match message {
            Message::Connect => {}
            Message::Disconnect => {}
            Message::Welcome { list } => *self.0.items().write() = list,
            Message::Add { id, label } => self.0.items().write().push(Item {
                id,
                label,
                completed: false,
            }),
            Message::Remove { id } => {
                self.0.items().write().retain(|item| item.id != id);
            }
            Message::MarkComplete { id, completed } => {
                if let Some(item) = self.find(&id) {
                    *item.completed().write() = completed
                }
            }
            Message::Edit { id, new_label } => {
                if let Some(item) = self.find(&id) {
                    *item.label().write() = new_label;
                }
            }
        }
    }

    fn find(&self, id: &Uuid) -> Option<Field<Item>> {
        let store = self.0.items().read_untracked();
        store
            .iter()
            .position(|item| &item.id == id)
            .map(|idx| self.0.items().at_unkeyed(idx).into())
    }
}
