use crate::extern_functions::*;
use serde::{Deserialize, Serialize};

use crate::helpers::{run_value_unit, Error};

pub struct Toast;

impl Toast {
    /// Show a toast asynchronously
    pub async fn show(options: impl Into<ShowOptions>) -> Result<(), Error> {
        run_value_unit(options, toast_show).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShowOptions {
    pub text: String,
}

impl From<&str> for ShowOptions {
    fn from(value: &str) -> Self {
        ShowOptions {
            text: value.to_string(),
        }
    }
}

impl From<String> for ShowOptions {
    fn from(text: String) -> Self {
        ShowOptions { text }
    }
}
