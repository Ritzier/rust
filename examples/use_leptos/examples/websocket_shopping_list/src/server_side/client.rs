use futures::channel::mpsc::UnboundedSender;
use leptos::prelude::{ReadValue, ServerFnError, StoredValue};
use uuid::Uuid;

use super::*;

#[derive(Clone, Copy)]
pub struct Client {
    pub store: State,
    connection: StoredValue<UnboundedSender<Result<MessageWithUser, ServerFnError>>>,
    user: Uuid,
}

impl Client {
    pub fn new(connection: UnboundedSender<Result<MessageWithUser, ServerFnError>>) -> Self {
        // Create a uuid for client
        let user = Uuid::new_v4();
        connection
            .unbounded_send(Ok((user, Message::Connect)))
            .unwrap();

        Self {
            user,
            store: State(Store::new(ShoppingList::default())),
            connection: StoredValue::new(connection),
        }
    }

    pub fn goodbye(&self) {
        _ = self
            .connection
            .read_value()
            .unbounded_send(Ok((self.user, Message::Disconnect)));
    }

    pub fn update(&self, message: Message) {
        self.store.apply_local_update(message.clone());
        self.send_update(message);
    }

    pub fn receive_update(&self, user: Uuid, message: Message) {
        match message {
            Message::Welcome { list } => {
                *self.store.0.items().write() = list;
            }
            _ => {
                if user != self.user {
                    self.store.apply_local_update(message);
                }
            }
        }
    }

    pub fn send_update(&self, message: Message) {
        self.connection
            .read_value()
            .unbounded_send(Ok((self.user, message)))
            .unwrap();
    }
}
