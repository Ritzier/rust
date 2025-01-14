#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use hackernews::*;
    use leptos::config::get_configuration;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::compression::{
        predicate::{NotForContentType, SizeAbove},
        CompressionLayer, CompressionLevel, Predicate,
    };

    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let predicate = SizeAbove::new(1_500) // files smaller than 1501 bytes are not compressed, since the MTU (Maximum Transmission Unit) of a TCP packet is 1500 bytes
        .and(NotForContentType::GRPC)
        .and(NotForContentType::IMAGES)
        // prevent compressing assets that are already statically compressed
        .and(NotForContentType::const_new("application/javascript"))
        .and(NotForContentType::const_new("application/wasm"))
        .and(NotForContentType::const_new("text/css"));

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(
            CompressionLayer::new()
                .quality(CompressionLevel::Best)
                .compress_when(predicate),
        )
        .fallback(fallback::file_and_error_handler)
        .with_state(leptos_options);

    // run our app wtih hyper
    leptos::logging::log!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
