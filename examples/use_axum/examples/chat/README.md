# Chat

## Server Side

AppState

```rust
struct AppState {
    tx: tokio::sync::broadcast::Sender<String>
}
```

- tx: Handler subscribed clients action

**websocket** function:

```rs
websocket(stream: WebSocket, state: Arc<AppState>)
```

### 1. `WebSocket`

`stream` is the websocket connection, it wrap a `sender` and `receiver` that directly communicate with the client and
server

```rs
let (mut sender, mut receiver) = stream.split();
```

- `sender`: Used to send data to the client
- `receiver`: Used to receive data to the client

### 2. Wait and Process data from `receiver`

```rs
 // Username gets set in the receive loop, if it's valid
let mut username = String::new();
// Loop until a text message is found
while let Some(Ok(message)) = receiver.next().await {
    if let Message::Text(name) = message {
        // If username that is sent by client is not taken, fill username string
        check_username(&state, &mut username, name.as_str()).await;

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
```

`receiver.next().await()`:

- `.next()` method is called on the stream, and `.await` cause it's a asynchronous. If not a message is sent, it waits
  (without blocking the thread) until a message arrives or the connection is closed

###

1. Send Task (Subscribe and Forward to Client)

Listens for messages from broadcast channel `rx` and sends the messages to client via the WebSocket `sender`

- How:
  - Subscribe Broadcast `rx` (Broadcast Receiver Channel)
  - Yields for messages from Broadcast `rx`
  - When a message received, send it to the client via WebSocket `sender`
  - If sending fails (e.g., client disconnected), breaks the loop and ends the task.

```rs
// We subscribe *before* sending the "joined" mesggae, so that we will also
// display it to our client
let mut rx = state.tx.subscribe();

// Now sent the "joined" message to all subscribers
let msg = format!("{username} joined");
tracing::debug!("{msg}");
let _ = state.tx.send(msg);

// Spawn the first task that will receive broadcast messages and send text
// messages over the websocket to our client
let mut send_task = tokio::spawn(async move {
    while let Ok(msg) = rx.recv().await {
        // In any websocket error, break loop
        if sender.send(Message::text(msg)).await.is_err() {
            break;
        }
    }
});
```

2. Receive Task (Receive from Client and Forward to Broadcast)

Listens for messages from the client via the Websocket `receiver` and sends to the broadcast channel `tx` so all the
clients receive it (include the client who send it)

- How:
  - Clone Broadcast Sender
  - Yields for messages from Client via WebSocket `receiver`
  - When a message received, send it to the broadcast `tx`
  - If the client disconnects or an error occurs, the loop ends and the task finishes

```rs
// Clone things we want to pass (move) to the receiving tack
let tx = state.tx.clone();
let name = username.clone();

// Spawn a task that takes messages from the websocket, prepends the user
// name, and sends them to all broadcast subscribers.
let mut recv_task = tokio::spawn(async move {
    while let Some(Ok(Message::Text(text))) = receiver.next().await {
        // Add username before message
        let _ = tx.send(format!("{name}: {text}"));
    }
});
```

3. Task Coordination

```rs
tokio::select! {
    _ = &mut send_task => recv_task.abort(),
    _ = &mut recv_task => send_task.abort(),
};
```

- Waits until either task finished (e.g., client disconnect)
- Aborts the other task to clean up

###

## Client Side

**`WebSocket` Handlers**

```js
// Create a WebSocket connection
const websocket = new WebSocket("ws://localhost:3000/websocket");

// Event: When connection opened
websocket.onopen = function () {
  console.log("WebSocket opened");
};

// Event: When connection closed
websocket.onclose = function () {
  console.log("WebSocket connection closed");
};

// Event: Received data from socket
websocket.onmessage = function (e) {
  console.log("received message: " + e.data);
};

// Send a data to server
websocket.send("Fuck");

// Close the connection
websocket.close();
```
