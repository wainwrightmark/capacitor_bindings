use crate::extern_functions::*;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::helpers::*;

pub struct SplashScreen;

impl SplashScreen {
    /// Show the splash screen
    pub async fn show(options: impl Into<ShowOptions>) -> Result<(), Error> {
        run_value_unit(options, splash_show).await
    }

    /// Hide the splash screen
    pub async fn hide(options: impl Into<HideOptions>) -> Result<(), Error> {
        run_value_unit(options, splash_hide).await
    }
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct HideOptions {
    /// How long (in ms) to fade out. On Android, if using the Android 12 Splash Screen API, it's not being used. Use launchFadeOutDuration configuration option instead.
    pub fade_out_duration: f64,
}

impl From<f64> for HideOptions {
    fn from(fade_out_duration: f64) -> Self {
        Self { fade_out_duration }
    }
}

impl Default for HideOptions {
    fn default() -> Self {
        Self {
            fade_out_duration: 200.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ShowOptions {
    /// Whether to auto hide the splash after showDuration
    pub auto_hide: bool,
    /// How long (in ms) to fade in.
    pub fade_in_duration: f64,
    /// How long (in ms) to fade out.
    pub fade_out_duration: f64,
    /// How long to show the splash screen when autoHide is enabled (in ms)
    pub show_duration: f64,
}

impl Default for ShowOptions {
    fn default() -> Self {
        Self {
            auto_hide: true,
            fade_in_duration: 200.0,
            fade_out_duration: 200.0,
            show_duration: 3000.0,
        }
    }
}
