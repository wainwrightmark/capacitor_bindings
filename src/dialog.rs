use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use crate::{
    extern_functions::{alert, confirm, prompt},
    helpers::*,
};

use crate::error::Error;

pub struct Dialog;

impl Dialog {
    /// Show an alert dialog
    pub async fn alert(options: impl Into<AlertOptions>) -> Result<(), Error> {
        run_value_unit(options, alert).await
    }

    /// Show a prompt dialog
    pub async fn prompt(options: impl Into<PromptOptions>) -> Result<PromptResult, Error> {
        run_value_value(options, prompt).await
    }

    /// Show a confirmation dialog
    pub async fn confirm(options: impl Into<ConfirmOptions>) -> Result<ConfirmResult, Error> {
        run_value_value(options, confirm).await
    }
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase", default)]
pub struct AlertOptions {
    #[builder(setter(into))]
    /// Title of the dialog.
    pub title: String,
    #[builder(setter(into))]
    /// Message to show on the dialog.
    pub message: String,
    #[builder(setter(into))]
    /// Text to use on the action button.
    pub button_title: String,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PromptResult {
    /// Text entered on the prompt.
    pub value: String,
    /// Whether if the prompt was canceled or accepted.
    pub cancelled: bool,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase", default)]
pub struct PromptOptions {
    #[builder(setter(into))]
    /// Title of the dialog.
    pub title: String,
    #[builder(setter(into))]
    /// Message to show on the dialog.
    pub message: String,
    #[builder(setter(into))]
    /// Text to use on the positive action button.
    pub ok_button_title: String,
    #[builder(setter(into))]
    /// Text to use on the negative action button.
    pub cancel_button_title: String,
    #[builder(setter(into, strip_option), default)]
    /// Placeholder text for hints.
    pub input_placeholder: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Prepopulated text.
    pub input_text: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ConfirmResult {
    /// true if the positive button was clicked, false otherwise.
    pub value: bool,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase", default)]
pub struct ConfirmOptions {
    #[builder(setter(into))]
    /// Title of the dialog.
    pub title: String,
    #[builder(setter(into))]
    /// Message to show on the dialog.
    pub message: String,
    #[builder(setter(into))]
    /// Text to use on the positive action button.
    pub ok_button_title: String,
    #[builder(setter(into))]
    /// Text to use on the negative action button.
    pub cancel_button_title: String,
}
