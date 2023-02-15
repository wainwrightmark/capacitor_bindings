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

impl From<&str> for ShowOptions {
    fn from(value: &str) -> Self {
        ShowOptions {
            text: value.to_string(),
        }
    }
}

impl From<String> for ShowOptions {
    fn from(text: String) -> Self {
        ShowOptions {
            text,
        }
    }
}

pub struct Toast;

impl Toast {
    /// Show a toast asynchronously
    pub async fn show(options: impl Into<ShowOptions>) {
        let options: ShowOptions = options.into();
        let js_val = serde_wasm_bindgen::to_value(&options).unwrap();
        show(js_val).await
    }
}
