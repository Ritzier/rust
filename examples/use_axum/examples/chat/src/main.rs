use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{
        ws::{Message, Utf8Bytes, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Html, IntoResponse},
};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    // We require unique usernames. This tracks which usernames have been taken
    user_set: Mutex<HashSet<String>>,
    // Channel used to send messages to all connected clients
    tx: broadcast::Sender<String>,
}

impl AppState {
    fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self {
            user_set: Mutex::new(HashSet::new()),
            tx,
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = AppState::new();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time
    let (mut sender, mut receiver) = stream.split();

    // Username gets set in the receive loop, if it's valid
    let mut username = String::new();
    // loop until a text message is found
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            // If username that is sent by client is no taken, fill username string
            check_username(&state, &mut username, name.as_str());

            // If not empty we want to quit the loop else we want to quit function
            if !username.is_empty() {
                break;
            } else {
                // Only send our client that username is taken
                let _ = sender
                    .send(Message::Text(Utf8Bytes::from_static(
                        "Username already taken.",
                    )))
                    .await;

                return;
            }
        }
    }

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client
    let mut send_taskj = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop
            if sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });
}

fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());

        string.push_str(name);
    }
}

// Include utf-8 file at **compile** time
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../chat.html"))
}
