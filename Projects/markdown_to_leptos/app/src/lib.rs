use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, Link, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod pages;
use pages::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HashedStylesheet options=options.clone() />
                <HydrationScripts options />
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
        <Title text="Welcome to Leptos!" />
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Stylesheet id="leptos" href="/pkg/markdown_to_leptos.css" />

        <Router>
            <main>
                <Routes fallback=|| "Page not found".into_view()>
                    <BlogRoute />
                    <Route path=path!("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}
