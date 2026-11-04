#[cfg(any(feature = "ios", feature = "android"))]
use crate::error::Error;
#[cfg(any(feature = "ios", feature = "android"))]
use crate::helpers::*;
use serde::{Deserialize, Serialize};

#[cfg(any(feature = "ios", feature = "android"))]
use crate::extern_functions::*;

/// The SystemBars API provides methods for configuring the style and visibility of the device System Bars / Status Bar.
/// This plugin is bundled with @capacitor/core.
pub struct SystemBars;

impl SystemBars {
    #[cfg(any(feature = "ios", feature = "android"))]
    /// Show the system bars.
    pub async fn show() -> Result<(), Error> {
        run_unit_unit(system_bars_show).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    /// Hide the system bars.
    pub async fn hide() -> Result<(), Error> {
        run_unit_unit(system_bars_hide).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    /// Set the current style of the system bars.
    pub async fn set_style(options: impl Into<StyleOptions>) -> Result<(), Error> {
        run_value_unit(options, system_bars_set_style).await
    }

    #[cfg(any(feature = "ios"))]
    /// Set the current style of the system bars.
    pub async fn set_animation(options: impl Into<AnimationOptions>) -> Result<(), Error> {
        run_value_unit(options, system_bars_set_animation).await
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimationOptions {
    #[serde(rename = "animation")]
    /// The type of status bar animation used when showing or hiding. This option is only supported on iOS.
    pub animation: SystemBarsAnimation,
}

impl Into<AnimationOptions> for SystemBarsAnimation {
    fn into(self) -> AnimationOptions {
        AnimationOptions { animation: self }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StyleOptions {
    #[serde(rename = "style")]
    /// Style of the text of the status bar.
    pub style: Style,

    #[serde(rename = "bar")]
    ///The system bar to which to apply the style.
    pub bar: SystemBarType,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]

pub enum SystemBarType {
    /// The top status bar on both Android and iOS.
    #[serde(rename = "StatusBar")]
    StatusBar,
    /// The navigation bar (or gesture bar on iOS) on both Android and iOS.
    #[serde(rename = "NavigationBar")]
    NavigationBar,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SystemBarsAnimation {
    None,
    Fade,
}
