use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::helpers::*;

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["Capacitor", "Plugins", "Dialog"], js_name="alert" )]
    async fn alert(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch,js_namespace = ["Capacitor", "Plugins", "Dialog"], js_name="prompt" )]
    async fn prompt(options: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch,js_namespace = ["Capacitor", "Plugins", "Dialog"], js_name="confirm" )]
    async fn confirm(options: JsValue) -> Result<JsValue, JsValue>;

}

pub struct Dialog;

impl Dialog {
    /// Show an alert dialog
    pub async fn alert(options: impl Into<AlertOptions>) -> Result<(), Error> {
        run_value_unit(options, alert).await
    }

    /// Show a prompt dialog
    pub async fn prompt(options: impl Into<PromptOptions>) -> Result<PromptResult, Error> {
        run_value_value(options, prompt).await
    }

    /// Show a confirmation dialog
    pub async fn confirm(options: impl Into<ConfirmOptions>) -> Result<ConfirmResult, Error> {
        run_value_value(options, confirm).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AlertOptions {
    /// Title of the dialog.
    pub title: String,
    /// Message to show on the dialog.
    pub message: String,
    /// Text to use on the action button.
    pub button_title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PromptResult {
    /// Text entered on the prompt.
    pub value: String,
    /// Whether if the prompt was canceled or accepted.
    pub cancelled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PromptOptions {
    /// Title of the dialog.
    pub title: String,
    /// Message to show on the dialog.
    pub message: String,
    /// Text to use on the positive action button.
    pub ok_button_title: String,
    /// Text to use on the negative action button.
    pub cancel_button_title: String,
    /// Placeholder text for hints.
    pub input_placeholder: Option<String>,
    /// Prepopulated text.
    pub input_text: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmResult {
    /// true if the positive button was clicked, false otherwise.
    pub value: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmOptions {
    /// Title of the dialog.
    pub title: String,
    /// Message to show on the dialog.
    pub message: String,
    /// Text to use on the positive action button.
    pub ok_button_title: String,
    /// Text to use on the negative action button.
    pub cancel_button_title: String,
}
