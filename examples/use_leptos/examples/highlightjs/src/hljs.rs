use gloo_utils::format::JsValueSerdeExt;
use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen(module = "/node_modules/@highlightjs/cdn-assets/es/highlight.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = "default", thread_local_v2)]
    static HIGHLIGHT_JS: JsValue;
}

/// Highlights code using highlight.js via WASM.
///
/// # Arguments
/// * `code` - The code to highlight.
/// * `language` - The language to use for highlighting.
///
/// # Returns
/// * `Ok(String)` - The highlighted HTML string.
/// * `Err(JsValue)` - If something goes wrong.
pub fn highlight(code: &str, language: &str) -> Result<String, JsValue> {
    let options = Object::new();
    Reflect::set(&options, &"language".into(), &language.into())
        .map_err(|e| JsValue::from(format!("Failed to assign lang: {:?}", e)))?;

    // Use .with() to access the inner JsValue
    HIGHLIGHT_JS.with(|highlight_js| {
        let highlight_fn = Reflect::get(highlight_js, &JsValue::from_str("highlight"))
            .map_err(|e| JsValue::from(format!("Failed to get highlight fn: {:?}", e)))?;

        let result =
            Function::from(highlight_fn).call2(highlight_js, &JsValue::from_str(code), &options)?;

        let value = Reflect::get(&result, &"value".into())
            .map_err(|e| JsValue::from(format!("Failed to get value: {:?}", e)))?;

        value
            .into_serde()
            .map_err(|e| JsValue::from(format!("Serde error: {:?}", e)))
    })
}
