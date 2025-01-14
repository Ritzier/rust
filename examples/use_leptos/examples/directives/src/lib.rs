use leptos::ev::click;
use leptos::prelude::*;
use web_sys::Element;

// no extra parameter
fn highlight(el: Element) {
    let mut highlighted = false;

    let handle = el.clone().on(click, move |_| {
        highlighted = !highlighted;

        if highlighted {
            el.style(("background-color", "yellow"));
        } else {
            el.style(("background-color", "transparent"));
        }
    });
    on_cleanup(move || drop(handle));
}

// on extra parameter
fn coipy_to_clipboard(el: Element, content: &str) {
    let content = content.to_string();
    let handle = el.clone().on(click, move |evt| {
        evt.prevent_default();
        evt.stop_propagation();

        let _ = window().navigator().clipboard().write_text(&content);

        el.set_inner_html(&format!("Copied \"{}\"", &content));
    });
    on_cleanup(move || drop(handle))
}

// custom paramter
#[derive(Clone)]
struct Amount(usize);

// a `default` value if no value is passed in
impl From<()> for Amount {
    fn from(_: ()) -> Self {
        Self(1)
    }
}

fn add_dot(el: Element, amount: Amount) {
    use leptos::wasm_bindgen::JsCast;
    let el = el.unchecked_into::<web_sys::HtmlElement>();

    let handle = el.clone().on(click, move |_| {
        el.set_inner_text(&format!("{}{}", el.inner_text(), ".".repeat(amount.0)))
    });
    on_cleanup(move || drop(handle))
}

#[component]
fn SomeComponent() -> impl IntoView {
    view! {
        <p>"Some paragraphs"</p>
        <p>"That can be clicked"</p>
        <p>"In order to highlight them"</p>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let data = "Hello World!";

    view! {
        <a href="#" use:copy_to_clipboard=data>
            "Copy \""
            {data}
            "\" to clipboard"
        </a>
    }
}
