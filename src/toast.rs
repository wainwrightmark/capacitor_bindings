use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "Toast"], js_name="show" )]
    async fn show(options: JsValue);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShowOptions {
    pub text: String,
}

pub struct Toast;

impl Toast {
    pub fn show(options: &ShowOptions) {
        let js_val = serde_wasm_bindgen::to_value(options).unwrap();
        wasm_bindgen_futures::spawn_local(show(js_val));
    }
}