use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "Share"], js_name="canShare" )]
    async fn can_share() -> JsValue;

    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "Share"], js_name="share" )]
    async fn share(options: JsValue) -> JsValue;
}

pub struct Share;

impl Share {
    pub async fn can_share() -> CanShareResult {
        let result = can_share().await;
        serde_wasm_bindgen::from_value(result)
            .expect("Should be able to deserialize CanShareResult")
    }

    pub async fn share(options: impl Into<ShareOptions>) -> ShareResult {
        let options = options.into();
        let options_js = serde_wasm_bindgen::to_value(&options)
            .expect("Should be able to serialize ShareOptions");

        let result = share(options_js).await;
        serde_wasm_bindgen::from_value(result).expect("Should be able to deserialize ShareResult")
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CanShareResult {
    /// Whether sharing is supported or not.
    pub value: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShareResult {
    /// 	Identifier of the app that received the share action. Can be an empty string in some cases. On web it will be undefined.
    pub activity_type: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShareOptions {
    /// Set a title for any message. This will be the subject if sharing to email
    pub title: Option<String>,
    /// Set some text to share
    pub text: Option<String>,
    /// Set a URL to share, can be http, https or file:// URL
    pub url: Option<String>,
    /// Set a title for the share modal. This option is only supported on Android.
    pub dialog_title: Option<String>,
    /// Array of file:// URLs of the files to be shared. Only supported on iOS and Android.
    pub files: Option<Vec<String>>,
}
