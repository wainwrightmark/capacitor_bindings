use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "ActionSheet"], js_name="showActions" )]
    async fn show_actions(options: JsValue) -> JsValue;
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShowActionsOptions {
    /// The title of the Action Sheet.
    pub title: String,
    // A message to show under the title. This option is only supported on iOS.
    pub message: Option<String>,
    /// Options the user can choose from.
    pub options: Vec<ActionSheetButton>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActionSheetButton {
    /// The title of the option
    pub title: String,

    /// Icon for the option (ionicon naming convention) This option is only supported on Web.
    pub icon: Option<String>,

    pub style: Option<ActionSheetButtonStyle>
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActionSheetButtonStyle{
    #[default]
    // Default Style of the option.
    Default,
    /// Style to use on destructive options.
    Destructive,
    /// Style to use on the option that cancels the Action Sheet. If used, should be on the latest available option.
    Cancel,

}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShowActionsResult {
    /// The index of the clicked option (Zero-based)
    pub index: u32,
}

pub struct ActionSheet;

impl ActionSheet {
    pub async fn show_actions(options: &ShowActionsOptions) -> ShowActionsResult {
        let js_val = serde_wasm_bindgen::to_value(options)
            .expect("Should be able to convert ShowActionsOptions to JsValue");

        let result_js_value = show_actions(js_val).await;

        let result: ShowActionsResult = serde_wasm_bindgen::from_value(result_js_value)
            .expect("Should be able to convert JsValue to ShowActionsResult");
        result
    }
}
