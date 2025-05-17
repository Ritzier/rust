use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use reactive_stores::Field;
use uuid::Uuid;

use crate::server_side::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HashedStylesheet options=options.clone() />
                <HydrationScripts options />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
                <link rel="stylesheet" id="leptos" href="/pkg/websocket_shopping_list.css" />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    use futures::{channel::mpsc, StreamExt};

    let (tx, rx) = mpsc::unbounded();

    let client = Client::new(tx);

    if cfg!(feature = "hydrate") {
        on_cleanup(move || {
            client.goodbye();
        });

        leptos::task::spawn_local(async move {
            match websocket_message(rx.into()).await {
                Ok(mut messages) => {
                    while let Some(msg) = messages.next().await {
                        match msg {
                            Ok((user, msg)) => {
                                client.receive_update(user, msg);
                            }
                            Err(e) => {
                                leptos::logging::error!("{e:?}");
                            }
                        }
                    }
                }
                Err(e) => leptos::logging::error!("{e}"),
            }
        });
    }

    let add_item = NodeRef::new();

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
fn ItemEditor(client: Client, #[prop(into)] item: Field<Item>) -> impl IntoView {
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
                class:hdieen=move || !editing.get()
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
