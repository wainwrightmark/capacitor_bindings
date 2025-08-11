use crate::extern_functions::*;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::helpers::*;

#[cfg(feature = "safe_area_plugin")]
pub struct SafeArea;

#[cfg(feature = "safe_area_plugin")]
impl SafeArea {
    pub fn enable(options: impl Into<Options>) -> Result<(), Error> {
        run_value_unit_sync(options, safe_area_enable)
    }

    pub fn disable(options: impl Into<Options>) -> Result<(), Error> {
        run_value_unit_sync(options, safe_area_disable)
    }
}

#[cfg(feature = "safe_area_plugin")]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Options{
    pub config: Config
}

#[cfg(feature = "safe_area_plugin")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Config {
    /// Flag indicating that you are responsible for drawing the background color for the system bars. If false it will fallback to the default colors for the system bars.
    pub custom_colors_for_system_bars: bool,

    /// Specifies the background color of the status bar. Should be in the format #RRGGBB or #AARRGGBB. Will only have effect if customColorsForSystemBars is set to true.
    pub status_bar_color: String,

    /// Specifies the color of the content (i.e. icon color) in the status bar.
    pub status_bar_content: LightOrDark,

    /// Specifies the background color of the navigation bar. Should be in the format #RRGGBB or #AARRGGBB. Will only have effect if customColorsForSystemBars is set to true.
    pub navigation_bar_color: String,

    /// Specifies the color of the content (i.e. icon color) in the navigation bar.
    pub navigation_bar_content: LightOrDark,

    /// Specifies the offset to be applied to the safe area insets. This means that if the safe area top inset is 30px, and the offset specified is 10px, the safe area top inset will be exposed as being 40px. Usually you don't need this, but on iOS the safe area insets are mostly offset a little more by itself already. So you might want to compensate for that on Android. It's totally up to you. The offset will be applied if Edge-to-Edge mode is enabled only.
    pub offset: usize,
}

#[cfg(feature = "safe_area_plugin")]
impl Default for Config {
    fn default() -> Self {
        Self {
            custom_colors_for_system_bars: true,
            status_bar_color: "#000000".to_string(),
            status_bar_content: LightOrDark::Light,
            navigation_bar_color: "#000000".to_string(),
            navigation_bar_content: LightOrDark::Light,
            offset: 0,
        }
    }
}

#[cfg(feature = "safe_area_plugin")]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LightOrDark {
    #[default]
    Light,
    Dark,
}
