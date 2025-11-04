use leptos::prelude::*;

mod language;
use language::*;
use leptos_router::{LazyRoute, lazy_route};
use strum::IntoEnumIterator;

pub struct DemoPageView;

#[lazy_route]
impl LazyRoute for DemoPageView {
    fn data() -> Self {
        Self {}
    }

    fn view(_this: Self) -> AnyView {
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
        .into_any()
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
