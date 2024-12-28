use leptos::{logging, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
                <link rel="stylesheet" id="leptos" href="/pkg/action_form_error_handling.css" />
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
    // Provides context that manages stylesheets, titles, meta tags, etc.
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

#[server]
async fn do_something(should_error: Option<String>) -> Result<String, ServerFnError> {
    match should_error {
        Some(error) => Err(ServerFnError::ServerError(String::from(error))),
        None => Ok(String::from("Successful submit")),
    }
}

/// Renders the home page of your application
#[component]
fn HomePage() -> impl IntoView {
    let do_something_action = ServerAction::<DoSomething>::new();
    let value = Signal::derive(move || {
        do_something_action
            .value()
            .get()
            .unwrap_or_else(|| Ok(String::new()))
    });

    Effect::new_isomorphic(move |_| {
        logging::log!("Got value = {:?}", value.get());
    });

    view! {
        <h1>"Test the action form!"</h1>
        <ErrorBoundary fallback=move |error| { move || format!("{:#?}", error.get()) }>
            <pre>{value}</pre>
            <ActionForm action=do_something_action attr:class="form">
                <label>"Should error: " <input type="checkbox" name="should_error" /></label>
                <button type="submit">"Submit"</button>
            </ActionForm>
        </ErrorBoundary>
    }
}
