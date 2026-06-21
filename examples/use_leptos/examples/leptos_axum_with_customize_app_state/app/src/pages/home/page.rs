use leptos::prelude::*;
use leptos_router::{LazyRoute, lazy_route};

pub struct HomePage;

#[lazy_route]
impl LazyRoute for HomePage {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let (count, set_count) = signal(0);
        let on_click = move |_| {
            set_count.update(|count| {
                *count += 1;
                leptos::logging::log!("Update num: {count}");
            })
        };

        view! {
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: "{count}</button>
            <Number />
        }
        .into_any()
    }
}

#[component]
fn Number() -> impl IntoView {
    let number = Resource::new(|| {}, |_| async move { current_user().await });

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || match number.get() {
                Some(Ok(value)) => view! { <p>{format!("Number: {}", value)}</p> }.into_any(),
                Some(Err(_)) => view! { <p>"Error loading number"</p> }.into_any(),
                None => view! { <p>"Loading..."</p> }.into_any(),
            }}

        </Suspense>
    }
}

#[server]
#[lazy]
pub async fn current_user() -> Result<u8, ServerFnError> {
    use std::sync::atomic::Ordering;

    use crate::app_state::AppState;

    // insert a simulated wait
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let app_state = expect_context::<AppState>();
    let prev = app_state.number.fetch_add(1, Ordering::Relaxed);

    Ok(prev)
}
