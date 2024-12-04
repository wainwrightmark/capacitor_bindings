use crate::error::Error;
use crate::helpers::*;
use crate::{extern_functions::*, prelude::*};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub struct Browser;

impl Browser {
    /// Open a page with the specified options.
    pub async fn open(options: impl Into<OpenOptions>) -> Result<(), Error> {
        run_value_unit(options, browser_open).await
    }
    #[cfg(any(feature = "ios", feature = "web"))]
    /// Web & iOS only: Close an open browser window.
    pub async fn close() -> Result<(), Error> {
        run_unit_unit(browser_close).await
    }

    /// Remove all native listeners for this plugin.
    pub async fn remove_all_listeners() -> Result<(), Error> {
        run_unit_unit(browser_remove_all_listeners).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    pub async fn add_browser_finished_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "browserFinished", browser_add_listener).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    pub async fn add_browser_page_loaded_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "browserPageLoaded", browser_add_listener).await
    }
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct OpenOptions {
    /// The URL to which the browser is opened.
    pub url: String,
    /// Web only: Optional target for browser open. Follows the target property for window.open. Defaults to _blank. Ignored on other platforms.
    pub window_name: Option<String>,
    /// A hex color to which the toolbar color is set.
    pub toolbar_color: Option<String>,
    /// iOS only: The presentation style of the browser. Defaults to fullscreen. Ignored on other platforms.
    pub presentation_style: Option<PresentationStyle>,
    /// iOS only: The width the browser when using presentationStyle 'popover' on iPads. Ignored on other platforms.
    pub width: Option<u32>,
    /// iOS only: The height the browser when using presentationStyle 'popover' on iPads. Ignored on other platforms.
    pub height: Option<u32>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PresentationStyle {
    Fullscreen,
    Popover,
}
