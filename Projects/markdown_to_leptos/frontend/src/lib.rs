#[wasm_bindgen::prelude::wasm_bindgen]
#[cfg(feature = "hydrate")]
pub fn hydrate() {
    use app::*;
    leptos::mount::hydrate_body(App);
}
