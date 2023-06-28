use crate::extern_functions::*;
use crate::helpers::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowActionsOptions {
    /// The title of the Action Sheet.
    pub title: String,
    // A message to show under the title. This option is only supported on iOS.
    pub message: Option<String>,
    /// Options the user can choose from.
    pub options: Vec<ActionSheetButton>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSheetButton {
    /// The title of the option
    pub title: String,

    /// Icon for the option (ionicon naming convention) This option is only supported on Web.
    pub icon: Option<String>,

    pub style: Option<ActionSheetButtonStyle>,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActionSheetButtonStyle {
    #[default]
    // Default Style of the option.
    Default,
    /// Style to use on destructive options.
    Destructive,
    /// Style to use on the option that cancels the Action Sheet. If used, should be on the latest available option.
    Cancel,
}

#[skip_serializing_none]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowActionsResult {
    /// The index of the clicked option (Zero-based)
    pub index: u32,
}

pub struct ActionSheet;

impl ActionSheet {
    pub async fn show_actions(
        options: impl Into<ShowActionsOptions>,
    ) -> Result<ShowActionsResult, Error> {
        run_value_value(options, action_sheet_show_actions).await
    }
}
