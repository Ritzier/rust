use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, MetaTags, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod language;
use language::Language;
use strum::IntoEnumIterator;

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
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
                <link rel="stylesheet" id="leptos" href="/pkg/highlightjs.css" />
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
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=path!("") view=DemoPage />
            </Routes>
        </Router>
    }
}

#[component]
fn DemoPage() -> impl IntoView {
    let (code, set_code) = signal(String::new());
    let (language, set_language) = signal(Language::Rust);

    #[cfg(not(feature = "ssr"))]
    {
        use gloo_net::http::Request;

        Effect::new(move || {
            let url = language.get().to_url();
            tracing::info!("{url}");

            leptos::task::spawn_local(async move {
                match Request::get(&url).send().await {
                    Ok(resp) => {
                        if let Ok(text) = resp.text().await {
                            set_code.set(text)
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to fetch: {e:?}");
                    }
                }
            })
        });
    }

    view! {
        <div class="demo">
            <header class="header">
                <span class="title">"HighlightJS"</span>

                <SelectLanguage language set_language />
            </header>

            <main class="main">
                <textarea
                    class="code-write"
                    prop:value=code
                    on:input=move |ev| {
                        set_code.set(event_target_value(&ev));
                    }
                />

                <div class="divider" />

                <HighlightCode code language />

            </main>
        </div>
    }
}

#[component]
fn HighlightCode(code: ReadSignal<String>, language: ReadSignal<Language>) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let _ = language;
    }

    #[cfg(not(feature = "ssr"))]
    {
        let (inner, set_inner) = signal(String::new());
        use crate::hljs;
        Effect::new(
            move |_| match hljs::highlight(&code.get(), &language.get().to_lang()) {
                Ok(result) => set_inner.set(result),
                Err(_e) => set_inner.set(code.get()),
            },
        );

        view! {
            <pre class="code-block">
                <code class="hljs" inner_html=inner></code>
            </pre>
        }
    }

    #[cfg(feature = "ssr")]
    {
        view! {
            <pre class="code-block">
                <code class="hljs">{code.get()}</code>
            </pre>
        }
    }
}

#[component]
fn SelectLanguage(
    language: ReadSignal<Language>,
    set_language: WriteSignal<Language>,
) -> impl IntoView {
    view! {
        <select class="language-select" prop:value=move || { language.get().to_string() }>
            {Language::iter()
                .map(|lang| {
                    view! {
                        <option on:click=move |_| {
                            set_language.set(lang.clone());
                        }>{lang.to_string()}</option>
                    }
                })
                .collect_view()}
        </select>
    }
}
