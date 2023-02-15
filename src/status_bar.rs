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

impl StatusBar{
    pub fn show(){
        wasm_bindgen_futures::spawn_local(status_bar_show());
    }

    pub fn hide(){
        wasm_bindgen_futures::spawn_local(status_bar_hide());
    }

    pub fn set_style(options: &StyleOptions){
        let js_val = serde_wasm_bindgen::to_value(options).unwrap();
        wasm_bindgen_futures::spawn_local(set_status_bar_style(js_val));
    }

    /// Set the background color of the status bar.
    /// This method is only supported on Android.
    pub fn set_background_color(options: &BackgroundColorOptions){
        let js_val = serde_wasm_bindgen::to_value(options).unwrap();
        wasm_bindgen_futures::spawn_local(set_status_bar_background_color(js_val));
    }

    /// Set whether or not the status bar should overlay the webview to allow usage of the space underneath it.
    /// This method is only supported on Android.
    pub fn set_overlays_web_view(options: &SetOverlaysWebViewOptions){
        let js_val = serde_wasm_bindgen::to_value(options).unwrap();
        wasm_bindgen_futures::spawn_local(set_overlays_web_view(js_val));
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BackgroundColorOptions{
    /// A hex color to which the status bar color is set. This option is only supported on Android.
    pub color: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StyleOptions{
    #[serde(rename = "style")]
    /// Style of the text of the status bar.
    pub style : Style
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Style{
    /// Light text for dark backgrounds.
    Dark,
    /// Dark text for light backgrounds.
    Light,
    /// The style is based on the device appearance. If the device is using Dark mode, the statusbar text will be light. If the device is using Light mode, the statusbar text will be dark. On Android the default will be the one the app was launched with.
    Default
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetOverlaysWebViewOptions{
    /// Whether to overlay the status bar or not.
    pub overlay: bool
}