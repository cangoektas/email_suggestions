use email_suggestions::EmailSuggestion;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
    let example_suggestion =
        EmailSuggestion::new(String::from("hello@world.com"), String::from("world.com"));

    JsValue::from_serde(&example_suggestion).unwrap()
}
