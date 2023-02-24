#[cfg(any(feature = "ios", feature = "android"))]
use crate::helpers::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="show" )]
    async fn status_bar_show() -> Result<(), JsValue>;

    #[wasm_bindgen(catch,js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="hide" )]
    async fn status_bar_hide() -> Result<(), JsValue>;

    #[wasm_bindgen(catch,js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="setStyle" )]
    async fn set_status_bar_style(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch,js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="setBackgroundColor" )]
    async fn set_status_bar_background_color(options: JsValue) -> Result<(), JsValue>;

    /// Set whether or not the status bar should overlay the webview to allow usage of the space underneath it.
    /// This method is only supported on Android.
    #[wasm_bindgen(catch,js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="setOverlaysWebView" )]
    async fn set_overlays_web_view(options: JsValue) -> Result<(), JsValue>;
}

/// The StatusBar API Provides methods for configuring the style of the Status Bar, along with showing or hiding it.
/// These methods are not implemented on web
pub struct StatusBar;

impl StatusBar {
    #[cfg(any(feature = "ios", feature = "android"))]
    /// Show the status bar. On iOS, if the status bar is initially hidden and the initial style is set to UIStatusBarStyleLightContent, first show call might present a glitch on the animation showing the text as dark and then transition to light. It's recommended to use Animation.None as the animation on the first call.
    pub async fn show() -> Result<(), Error> {
        run_unit_unit(status_bar_show).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    /// Hide the status bar.
    pub async fn hide() -> Result<(), Error> {
        run_unit_unit(status_bar_hide).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    /// Set the current style of the status bar.
    pub async fn set_style(options: impl Into<StyleOptions>) -> Result<(), Error> {
        run_value_unit(options, set_status_bar_style).await
    }

    #[cfg(any(feature = "android"))]
    /// Set the background color of the status bar.
    /// This method is only supported on Android.
    pub async fn set_background_color(
        options: impl Into<BackgroundColorOptions>,
    ) -> Result<(), Error> {
        run_value_unit(options, set_status_bar_background_color).await
    }

    #[cfg(any(feature = "android"))]
    /// Set whether or not the status bar should overlay the webview to allow usage of the space underneath it.
    /// This method is only supported on Android.
    pub async fn set_overlays_web_view(
        options: impl Into<SetOverlaysWebViewOptions>,
    ) -> Result<(), Error> {
        run_value_unit(options, set_overlays_web_view).await
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
