use crate::extern_functions::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helpers::*;

pub struct ScreenReader;

impl ScreenReader {
    #[cfg(any(feature = "ios", feature = "android"))]
    /// Whether a Screen Reader is currently active.
    /// This method is not supported on web (it is not possible to detect Screen Readers).
    pub async fn is_enabled() -> Result<ScreenReaderState, Error> {
        run_unit_value(screen_reader_is_enabled).await
    }

    /// Text-to-Speech functionality.
    /// This function will only work if a Screen Reader is currently active.
    /// On web, browsers must support the SpeechSynthesis API, or this method will throw an error.
    /// For more text-to-speech capabilities, please see the Capacitor Community Text-to-Speech plugin.
    pub async fn speak(options: impl Into<SpeakOptions>) -> Result<(), Error> {
        run_value_unit(options, screen_reader_speak).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    /// Add a listener for when the screen reader is turned on or off.
    /// This event used to be named 'accessibilityScreenReaderStateChange'.
    /// This method is not supported on web (it is not possible to detect Screen Readers).
    pub async fn add_state_change_listener<F: Fn(ScreenReaderState) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "stateChange", network_add_listener).await
    }
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SpeakOptions {
    /// The text to speak.
    pub value: String,
    /// The language to speak the text in, as its ISO 639-1 Code (e.g.: "en"). This option is only supported on Android.
    pub language: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ScreenReaderState {
    /// Whether a Screen Reader is currently active.
    pub value: bool,
}

impl From<&str> for SpeakOptions {
    fn from(value: &str) -> Self {
        SpeakOptions {
            value: value.to_string(),
            language: None,
        }
    }
}

impl From<String> for SpeakOptions {
    fn from(value: String) -> Self {
        SpeakOptions {
            value,
            language: None,
        }
    }
}
