use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::helpers::*;

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["Capacitor", "Plugins", "Share"], js_name="canShare" )]
    async fn can_share() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["Capacitor", "Plugins", "Share"], js_name="share" )]
    async fn share(options: JsValue) -> Result<JsValue, JsValue>;
}

/// The Share API provides methods for sharing content in any sharing-enabled apps the user may have installed.
/// The Share API works on iOS, Android, and the Web (using the new Web Share API), though web support is currently spotty.
pub struct Share;

impl Share {
    /// Check if sharing is supported.
    pub async fn can_share() -> Result<CanShareResult, Error> {
        run_unit_value(can_share).await
    }

    /// Show a Share modal for sharing content with other apps
    pub async fn share(options: impl Into<ShareOptions>) -> Result<ShareResult, Error> {
        run_value_value(options, share).await
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
    pub activity_type: Option<String>,
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
