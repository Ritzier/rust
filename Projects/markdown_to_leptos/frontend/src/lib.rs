#[wasm_bindgen::prelude::wasm_bindgen]
#[cfg(feature = "hydrate")]
pub fn hydrate() {
    leptos::mount::hydrate_islands();
}
