# Setup AppState on Leptos Axum

## AppState

Requires crate `axum` with features `macros`:

```Cargo.toml
axum = { version = "...", features = ["macros"] }
```

`Arc<AtomicU8>` is used because `AppState` must be `Clone`, and `AtomicU8` is not `Clone` on its own.

```rust
#[derive(FromRef, Clone)]
pub struct AppState {
    pub number: Arc<AtomicU8>,
    pub leptos_options: LeptosOptions,
}
```

## Steup (leptos_axum)

The turbofish `::<AppState, _>` on `file_and_error_handler` is required so the compiler can resolve
`LeptosOptions: FromRef<AppState>`.

```rust
let app_state = AppState {
    number: Arc::new(AtomicU8::new(10)),
    leptos_options,
};

let app = Router::new()
    .leptos_routes_with_context(
        &app_state,
        routes,
        {
            let app_state = app_state.clone();
            move || provide_context(app_state.clone())
        },
        App,
    )
    .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
    .with_state(app_state);
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
