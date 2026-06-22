# Setup AppState on Leptos Axum

## AppState

Requires crate `axum` with features `macros`:

```Cargo.toml
axum = { version = "...", features = ["macros"] }
```

`Arc<AtomicU8>` is used because `AppState` must be `Clone`, and `AtomicU8` is not `Clone` on its own.

Note: `AppState` no longer needs to embed `LeptosOptions` or derive `FromRef<LeptosOptions>`. `LeptosOptions` is passed
directly as the Axum router state instead.

```rust
#[derive(Clone)]
pub struct AppState {
    pub number: Arc<AtomicU8>,
}
```

## Setup (leptos_axum)

`AppState` is injected into the request context via `leptos_routes_with_context`. `LeptosOptions` is used directly as
the router state, so the `FromRef` turbofish is typed accordingly.

```rust
let app_state = AppState {
    number: Arc::new(AtomicU8::new(10)),
};

let app = Router::new()
    .leptos_routes_with_context(
        &leptos_options,
        routes,
        move || provide_context(app_state.clone()),
        {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        },
    )
    .fallback(leptos_axum::file_and_error_handler::<LeptosOptions, _>(
        shell,
    ))
    .with_state(leptos_options);
```

## Usage

Only available inside `#[server]` functions. Context is **not** available on the client (WASM) — use a `Resource` +
server function to pass data to the client instead.

```rust
#[server]
pub async fn do_something() -> Result<(), ServerFnError> {
    let app_state = expect_context::<AppState>();
    let n = app_state.number.load(Ordering::Relaxed);
    Ok(())
}
```
