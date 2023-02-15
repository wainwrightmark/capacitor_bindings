use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen()]
extern "C" {
    // STATUS BAR
    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="show" )]
    async fn status_bar_show();

    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="hide" )]
    async fn status_bar_hide();

    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="setStyle" )]
    async fn set_status_bar_style(options: JsValue);

    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="setBackgroundColor" )]
    async fn set_status_bar_background_color(options: JsValue);

    /// Set whether or not the status bar should overlay the webview to allow usage of the space underneath it.
    /// This method is only supported on Android.
    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="setOverlaysWebView" )]
    async fn set_overlays_web_view(options: JsValue);
}

pub struct StatusBar;

impl StatusBar {
    pub async fn show() {
        status_bar_show().await;
    }

    pub async fn hide() {
        status_bar_hide().await;
    }

    pub async fn set_style(options: impl Into<StyleOptions>) {
        let options = options.into();
        let js_val = serde_wasm_bindgen::to_value(&options).unwrap();
        set_status_bar_style(js_val).await;
    }

    /// Set the background color of the status bar.
    /// This method is only supported on Android.
    pub async fn set_background_color(options: impl Into<BackgroundColorOptions>) {
        let options = options.into();
        let js_val = serde_wasm_bindgen::to_value(&options).unwrap();
        set_status_bar_background_color(js_val).await;
    }

    /// Set whether or not the status bar should overlay the webview to allow usage of the space underneath it.
    /// This method is only supported on Android.
    pub async fn set_overlays_web_view(options: impl Into<SetOverlaysWebViewOptions>) {
        let options = options.into();
        let js_val = serde_wasm_bindgen::to_value(&options).unwrap();
        set_overlays_web_view(js_val).await;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BackgroundColorOptions {
    /// A hex color (e.g. #FF0000) to which the status bar color is set. This option is only supported on Android.
    pub color: String,
}

impl From<&str> for BackgroundColorOptions {
    fn from(value: &str) -> Self {
        Self {
            color: value.to_string(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct StyleOptions {
    #[serde(rename = "style")]
    /// Style of the text of the status bar.
    pub style: Style,
}

impl From<Style> for StyleOptions {
    fn from(style: Style) -> Self {
        Self { style }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Style {
    #[default]
    /// The style is based on the device appearance. If the device is using Dark mode, the statusbar text will be light. If the device is using Light mode, the statusbar text will be dark. On Android the default will be the one the app was launched with.
    Default,
    /// Light text for dark backgrounds.
    Dark,
    /// Dark text for light backgrounds.
    Light,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct SetOverlaysWebViewOptions {
    /// Whether to overlay the status bar or not.
    pub overlay: bool,
}

impl From<bool> for SetOverlaysWebViewOptions {
    fn from(overlay: bool) -> Self {
        Self { overlay }
    }
}
