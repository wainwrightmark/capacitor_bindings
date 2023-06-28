use crate::extern_functions::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use crate::helpers::*;
use crate::{error::Error};

/// The Share API provides methods for sharing content in any sharing-enabled apps the user may have installed.
/// The Share API works on iOS, Android, and the Web (using the new Web Share API), though web support is currently spotty.
pub struct Share;

impl Share {
    /// Check if sharing is supported.
    pub async fn can_share() -> Result<CanShareResult, Error> {
        run_unit_value(share_can_share).await
    }

    /// Show a Share modal for sharing content with other apps
    pub async fn share(options: impl Into<ShareOptions>) -> Result<ShareResult, Error> {
        run_value_value(options, share_share).await
    }
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CanShareResult {
    /// Whether sharing is supported or not.
    pub value: bool,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ShareResult {
    /// 	Identifier of the app that received the share action. Can be an empty string in some cases. On web it will be undefined.
    pub activity_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase", default)]
pub struct ShareOptions {
    #[builder(setter(into, strip_option), default)]
    /// Set a title for any message. This will be the subject if sharing to email
    pub title: Option<String>,

    #[builder(setter(into, strip_option), default)]
    /// Set some text to share
    pub text: Option<String>,

    #[builder(setter(into, strip_option), default)]
    /// Set a URL to share, can be http, https or file:// URL
    pub url: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Set a title for the share modal. This option is only supported on Android.
    pub dialog_title: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Array of file:// URLs of the files to be shared. Only supported on iOS and Android.
    pub files: Option<Vec<String>>,
}
