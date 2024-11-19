# auto-reload

This examples shows how you can set up a development environment for your axum
service such that whenever the source code changes, the app is recompiled and
restarted. It uses `listenfd` to be able to migrate connections from and old
version of the app to a newly-compiled version.

## Setup

```sh
cargo install cargo-watch systemfd
```

## Running

```sh
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant CW as cargo-watch
    participant SF as systemfd
    participant App as Application
    participant Port as Port 3000

    SF->>Port: Open and listen on port 3000
    CW->>App: Compile and run initial version
    SF->>App: Pass open socket
    App->>Port: Start serving requests

    loop Development Cycle
        Dev->>App: Make code changes
        CW->>App: Detect changes
        CW->>App: Recompile
        SF->>App: Keep port 3000 open
        CW->>App: Stop previous version
        CW->>App: Start new version
        SF->>App: Pass open socket to new version
        App->>Port: Continue serving requests
    end

```
