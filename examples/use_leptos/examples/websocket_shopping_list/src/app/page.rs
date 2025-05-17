use futures::channel::mpsc::UnboundedSender;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::server_fn::codec::JsonEncoding;
use leptos::server_fn::{BoxedStream, Websocket};
use leptos::task::spawn_local;
use reactive_stores::{ArcStore, Field, Store, StoreFieldIterator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type MessageWithUser = (Uuid, Message);

#[derive(Debug, Default, Clone, Store, PartialEq, Eq)]
pub struct ShoppingList {
    #[store(key: Uuid = |item| item.id)]
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Store, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub label: String,
    pub completed: bool,
}

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

#[derive(Debug, Clone, Copy)]
pub struct Client {
    store: State,
    connection: StoredValue<UnboundedSender<Result<MessageWithUser, ServerFnError>>>,
    user: Uuid,
}

#[derive(Debug, Clone, Copy)]
pub struct State(Store<ShoppingList>);

impl From<ArcStore<ShoppingList>> for State {
    fn from(value: ArcStore<ShoppingList>) -> Self {
        State(value.into())
    }
}

impl State {
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
                    *item.completed().write() = completed;
                }
            }
            Message::Edit { id, new_label } => {
                if let Some(item) = self.find(&id) {
                    *item.label().write() = new_label
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

impl Client {
    pub fn new(connection: UnboundedSender<Result<MessageWithUser, ServerFnError>>) -> Self {
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

    /// Updates the shopping list from this local devices. This will both
    /// uodate the state of the UI here, and send the update over the websocket
    pub fn update(&self, message: Message) {
        self.store.apply_local_update(message.clone());
        self.send_update(message);
    }

    /// Applies on update that was received from the server.
    pub fn received_update(&self, user: Uuid, message: Message) {
        match message {
            Message::Welcome { list } => *self.store.0.items().write() = list,
            _ => {
                if user != self.user {
                    self.store.apply_local_update(message);
                }
            }
        }
    }

    /// Sends one update to the server.
    pub fn send_update(&self, message: Message) {
        self.connection
            .read_value()
            .unbounded_send(Ok((self.user, message)))
            .unwrap()
    }
}

#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
async fn messages(
    input: BoxedStream<MessageWithUser, ServerFnError>,
) -> Result<BoxedStream<MessageWithUser, ServerFnError>, ServerFnError> {
    let mut input = input;

    #[derive(Debug, Clone)]
    pub struct ServerState(ArcStore<ShoppingList>);

    impl ServerState {
        fn initial_items(&self) -> Vec<Item> {
            self.0.clone().items().get_untracked()
        }

        fn apply_local_update(&self, message: Message) {
            Owner::new().with(|| State::from(self.0.clone()).apply_local_update(message))
        }
    }

    use futures::{
        channel::mpsc::{channel, Sender},
        StreamExt,
    };
    use std::{
        collections::HashMap,
        sync::{LazyLock, Mutex},
    };

    static SHOPPING_LIST: LazyLock<ServerState> =
        LazyLock::new(|| ServerState(ArcStore::new(ShoppingList::default())));
    static USER_SENDERS: LazyLock<
        Mutex<HashMap<Uuid, Sender<Result<MessageWithUser, ServerFnError>>>>,
    > = LazyLock::new(|| Mutex::new(HashMap::new()));

    let (tx, rx) = channel(32);
    let mut tx = Some(tx);

    tokio::spawn(async move {
        while let Some(msg) = input.next().await {
            match msg {
                Err(e) => eprintln!("{e}"),
                Ok((user, msg)) => match msg {
                    Message::Connect => {
                        println!("\nUser conncting: {user:?}");
                        if let Some(mut tx) = tx.take() {
                            tx.try_send(Ok((
                                user,
                                Message::Welcome {
                                    list: SHOPPING_LIST.initial_items(),
                                },
                            )))
                            .unwrap();
                            USER_SENDERS.lock().unwrap().insert(user, tx);
                        }
                    }

                    Message::Disconnect => {
                        println!("\nuser disconnecting: {user:?}");
                        USER_SENDERS.lock().unwrap().remove(&user);
                    }

                    _ => {
                        println!("\nmsg from {user:?} {msg:?}");

                        SHOPPING_LIST.apply_local_update(msg.clone());

                        let mut senders = USER_SENDERS.lock().unwrap();
                        senders.retain(|tx_user, tx| {
                            if tx_user != &user {
                                let res = tx.try_send(Ok((user, msg.clone())));
                                if res.is_err() {
                                    println!("user disconnected: {tx_user:?}");
                                    return false;
                                }
                            }
                            true
                        });

                        println!("\n{:#?}", &(SHOPPING_LIST.0.read_untracked()));
                    }
                },
            }
        }
    });

    Ok(rx.into())
}

#[component]
pub fn MainPage() -> impl IntoView {
    use futures::{channel::mpsc, StreamExt};
    let (tx, rx) = mpsc::unbounded();

    let client = Client::new(tx);

    if cfg!(feature = "hydrate") {
        on_cleanup(move || {
            client.goodbye();
        });

        spawn_local(async move {
            match messages(rx.into()).await {
                Ok(mut messages) => {
                    while let Some(msg) = messages.next().await {
                        leptos::logging::log!("{:?}", msg);
                        match msg {
                            Ok((user, msg)) => {
                                // when we get a message from the server, only paaly it locally
                                client.received_update(user, msg);
                            }
                            Err(e) => {
                                leptos::logging::error!("{e:?}")
                            }
                        }
                    }
                }
                Err(e) => leptos::logging::warn!("{e}"),
            }
        });
    }

    let add_item = NodeRef::<Input>::new();

    view! {
        <h1>"My Shopping List"</h1>
        <form
            class="add"
            on:submit:target=move |ev| {
                ev.prevent_default();
                let label = add_item.get().unwrap().value();
                client
                    .update(Message::Add {
                        id: Uuid::new_v4(),
                        label,
                    });
                ev.target().reset();
            }
        >
            <input type="text" node_ref=add_item autofocus />
            <input type="submit" value="Add" />
        </form>

        <ul>
            <For each=move || client.store.0.items() key=|item| item.id().get() let:item>
                <ItemEditor client item />
            </For>
        </ul>
    }
}

#[component]
pub fn ItemEditor(client: Client, #[prop(into)] item: Field<Item>) -> impl IntoView {
    let editing = RwSignal::new(false);

    view! {
        <li class:completed=item.completed()>
            <input
                class="item"
                type="checkbox"
                prop:checked=item.completed()
                id=move || item.id().read().to_string()
                on:change:target=move |ev| {
                    client
                        .update(Message::MarkComplete {
                            id: item.id().get(),
                            completed: ev.target().checked(),
                        })
                }
            />

            <label
                class="item"
                class:hidden=move || editing.get()
                for=move || item.id().read().to_string()
            >
                {item.label()}
            </label>

            <input
                class="item"
                type="text"
                prop:value=item.label()
                on:change:target=move |ev| {
                    client
                        .update(Message::Edit {
                            id: item.id().get(),
                            new_label: ev.target().value(),
                        });
                    editing.set(false);
                }
                class:hidden=move || !editing.get()
            />

            <button class:hidden=move || editing.get() on:click=move |_| editing.set(true)>
                "Edit"
            </button>
            <button class:hidden=move || !editing.get() on:click=move |_| editing.set(false)>
                "Cancel"
            </button>
            <button on:click=move |_| {
                client
                    .update(Message::Remove {
                        id: item.id().get(),
                    })
            }>"X"</button>
        </li>
    }
}
