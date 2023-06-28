#[cfg(any(feature = "ios", feature = "android"))]
use crate::helpers::*;
#[cfg(any(feature = "ios", feature = "android"))]
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[cfg(any(feature = "ios", feature = "android"))]
use crate::extern_functions::*;

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
        run_value_unit(options, status_bar_set_style).await
    }

    #[cfg(any(feature = "android"))]
    /// Set the background color of the status bar.
    /// This method is only supported on Android.
    pub async fn set_background_color(
        options: impl Into<BackgroundColorOptions>,
    ) -> Result<(), Error> {
        run_value_unit(options, status_bar_set_background_color).await
    }

    #[cfg(any(feature = "android"))]
    /// Set whether or not the status bar should overlay the webview to allow usage of the space underneath it.
    /// This method is only supported on Android.
    pub async fn set_overlays_web_view(
        options: impl Into<SetOverlaysWebViewOptions>,
    ) -> Result<(), Error> {
        run_value_unit(options, status_bar_set_overlays_web_view).await
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetOverlaysWebViewOptions {
    /// Whether to overlay the status bar or not.
    pub overlay: bool,
}

impl From<bool> for SetOverlaysWebViewOptions {
    fn from(overlay: bool) -> Self {
        Self { overlay }
    }
}
