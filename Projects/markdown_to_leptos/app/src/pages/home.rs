use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Home Page"</h1>
        <a href="/blog">"Go checkout my blog page!"</a>
    }
}
