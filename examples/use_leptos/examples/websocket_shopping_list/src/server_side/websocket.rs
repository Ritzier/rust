use leptos::prelude::*;
use leptos::server_fn::{codec::JsonEncoding, BoxedStream, Websocket};

use super::*;

#[cfg(feature = "ssr")]
mod ssr {
    use super::*;
    use std::collections::HashMap;
    use std::sync::LazyLock;

    use futures::channel::mpsc::Sender;
    use futures::lock::Mutex;

    type UserSender = Sender<Result<MessageWithUser, ServerFnError>>;

    pub static SHOPPING_LIST: LazyLock<ServerState> =
        LazyLock::new(|| ServerState(ArcStore::new(ShoppingList::default())));
    pub static USER_SENDERS: LazyLock<Mutex<HashMap<Uuid, UserSender>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    #[derive(Debug, Clone)]
    pub struct ServerState(pub ArcStore<ShoppingList>);

    impl ServerState {
        pub fn initial_items(&self) -> Vec<Item> {
            self.0.clone().items().get_untracked()
        }

        pub fn apply_local_undate(&self, message: Message) {
            Owner::new().with(|| State::from(self.0.clone()).apply_local_update(message))
        }
    }
}

#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
pub async fn websocket_message(
    input: BoxedStream<MessageWithUser, ServerFnError>,
) -> Result<BoxedStream<MessageWithUser, ServerFnError>, ServerFnError> {
    use futures::channel::mpsc::channel;
    use futures::StreamExt;
    use ssr::*;

    let mut input = input;

    let (tx, rx) = channel(32);
    let mut tx = Some(tx);

    tokio::spawn(async move {
        while let Some(msg) = input.next().await {
            match msg {
                Err(e) => eprintln!("{e}"),
                Ok((user, msg)) => match msg {
                    Message::Connect => {
                        println!("\nuser connecting: {user:?}");
                        if let Some(mut tx) = tx.take() {
                            tx.try_send(Ok((
                                user,
                                Message::Welcome {
                                    list: SHOPPING_LIST.initial_items(),
                                },
                            )))
                            .unwrap();
                            USER_SENDERS.lock().await.insert(user, tx);
                        }
                    }

                    Message::Disconnect => {
                        println!("\nuser disconnecting: {user:?}");
                        USER_SENDERS.lock().await.remove(&user);
                    }

                    _ => {
                        println!("\nmsg from {user:?}: {msg:?}");

                        SHOPPING_LIST.apply_local_undate(msg.clone());

                        let mut senders = USER_SENDERS.lock().await;
                        senders.retain(|tx_user, tx| {
                            if tx_user != &user && tx.try_send(Ok((user, msg.clone()))).is_err() {
                                println!("user disconneted: {tx_user:?}");
                                return false;
                            }
                            true
                        });

                        println!("\n{:#?}", &*SHOPPING_LIST.0.read_untracked());
                    }
                },
            }
        }
    });

    Ok(rx.into())
}
