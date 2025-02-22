use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[cfg(feature = "ssr")]
pub mod ssr_imports {
    use std::sync::{atomic::AtomicI32, LazyLock};

    pub use std::sync::atomic::Ordering;

    use tokio::sync::broadcast::{channel, Sender};

    pub static COUNT: AtomicI32 = AtomicI32::new(0);

    pub static COUNT_CHANNEL: LazyLock<Sender<i32>> = LazyLock::new(|| channel(64).0);
}

#[server]
pub async fn get_count() -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    Ok(COUNT.load(Ordering::Relaxed))
}

#[server]
pub async fn adjust_count(delta: i32) -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(new);

    Ok(new)
}

#[server]
pub async fn clear_count() -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    COUNT.store(0, Ordering::Relaxed);
    let _ = COUNT_CHANNEL.send(0);
    Ok(0)
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <link rel="stylesheet" id="leptos" href="/pkg/counter_isomorphic.css" />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
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
        <Title text="Welcome to Leptos" />

        <Router>
            <main>
                <Routes fallback=|| "Page not found".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let dec = Action::new(|_: &()| adjust_count(-1));
    let inc = Action::new(|_: &()| adjust_count(1));
    let clear = Action::new(|_: &()| clear_count());

    #[cfg(not(feature = "ssr"))]
    let value = {
        use futures::StreamExt;
        use send_wrapper::SendWrapper;

        let mut source = SendWrapper::new(
            gloo_net::eventsource::futures::EventSource::new("/api/event")
                .expect("couldn't connect to SSE stream"),
        );
        let s = ReadSignal::from_stream_unsync(source.subscribe("message").unwrap().map(|value| {
            match value {
                Ok(value) => value.1.data().as_string().expect("expected string value"),
                Err(_) => "0".to_string(),
            }
        }));

        on_cleanup(move || source.take().close());
        s
    };

    #[cfg(feature = "ssr")]
    let (value, _) = signal(None::<i32>);

    view! {
        <button on:click=move |_| {
            clear.dispatch(());
        }>"Clear"</button>
        <button on:click=move |_| {
            dec.dispatch(());
        }>"-1"</button>
        <button on:click=move |_| {
            inc.dispatch(());
        }>"+1"</button>
        <span>{move || value.get().unwrap_or_default()}</span>
    }
}
