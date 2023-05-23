use crate::extern_functions::*;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::helpers::{run_value_unit, Error};

pub struct Toast;

impl Toast {
    /// Show a toast asynchronously
    pub async fn show(options: impl Into<ShowOptions>) -> Result<(), Error> {
        run_value_unit(options, toast_show).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ShowOptions {
    #[builder(setter(into))]
    pub text: String,
    #[builder(default)]
    pub duration: ToastDuration,
    #[builder(default)]
    pub position: ToastPosition,
}

impl From<&str> for ShowOptions {
    fn from(value: &str) -> Self {
        ShowOptions {
            text: value.to_string(),
            position: Default::default(),
            duration: Default::default(),
        }
    }
}

impl From<String> for ShowOptions {
    fn from(text: String) -> Self {
        ShowOptions {
            text,
            position: Default::default(),
            duration: Default::default(),
        }
    }
}

/// Duration of the Toast
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ToastDuration {
    /// 2000ms
    #[default]
    Short,
    /// 3500ms
    Long,
}

/// Position of the Toast. On Android 12 and newer all toasts are shown at the bottom. On web all toasts are also shown at the bottom but this seems to be an oversight on Ionic's part.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ToastPosition {
    #[default]
    Bottom,
    Center,
    Top,
}
