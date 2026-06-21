use std::sync::Arc;
use std::sync::atomic::AtomicU8;

use app::app_state::AppState;
use app::*;
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};

use crate::Error;

pub struct Server;

impl Server {
    pub async fn setup() -> Result<(), Error> {
        let conf = get_configuration(None)?;
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options;
        let routes = generate_route_list(App);

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

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| Error::AdressUsed { addr, source: e })?;

        println!("Listening on: {addr:#?}");

        axum::serve(listener, app.into_make_service())
            .await
            .map_err(|e| Error::AdressUsed { addr, source: e })?;

        Ok(())
    }
}
