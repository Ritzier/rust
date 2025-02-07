use leptos::prelude::*;
use leptos_meta::MetaTags;

mod todo;
pub use todo::App;

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
                <link rel="stylesheet" id="leptos" href="/pkg/stores.css" />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
