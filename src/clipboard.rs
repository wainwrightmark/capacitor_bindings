use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use crate::{
    extern_functions::{read, write},
    helpers::*,
};

pub struct Clipboard;

impl Clipboard {
    /// Show an alert dialog
    pub async fn write(options: impl Into<WriteOptions>) -> Result<(), Error> {
        run_value_unit(options, write).await
    }

    /// Show a prompt dialog
    pub async fn read() -> Result<ReadResult, Error> {
        run_unit_value(read).await
    }
}

/// Represents the data to be written to the clipboard.
#[skip_serializing_none]
#[derive(TypedBuilder, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WriteOptions {
    #[builder(default, setter(strip_option, into))]
    /// Text value to copy.
    pub string: Option<String>,

    #[builder(default, setter(strip_option, into))]
    /// Image in Data URL format to copy.
    pub image: Option<String>,

    #[builder(default, setter(strip_option, into))]
    /// URL string to copy.
    pub url: Option<String>,

    #[builder(default, setter(strip_option, into))]
    /// User visible label to accompany the copied data (Android Only).
    pub label: Option<String>,
}

/// Represents the data read from the clipboard.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadResult {
    /// Data read from the clipboard.
    pub value: String,
    /// Type of data in the clipboard.
    pub r#type: String,
}