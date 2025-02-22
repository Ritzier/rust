use std::convert::Infallible;

use app::*;
use axum::{
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::get,
    Router,
};
use futures::{Stream, StreamExt};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

async fn counter_event() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    use app::{get_count, ssr_imports::*};

    let mut rx = COUNT_CHANNEL.subscribe();

    let stream = futures::stream::once(async move { get_count().await.unwrap_or_default() })
        .chain(async_stream::stream! {
            while let Ok(value) = rx.recv().await {
                yield value;
            }
        })
        .map(|value| Ok(Event::default().event("message").data(value.to_string())));

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/event", get(counter_event))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    leptos::logging::log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
