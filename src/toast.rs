use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::helpers::run_value_unit;

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "Toast"], js_name="show" )]
    async fn show(options: JsValue);
}

pub struct Toast;

impl Toast {
    /// Show a toast asynchronously
    pub async fn show(options: impl Into<ShowOptions>) {
        run_value_unit(options, show).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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


