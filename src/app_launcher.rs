use crate::extern_functions::*;
use crate::helpers::*;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CanOpenUrlOptions {
    pub url: String,
}

impl Into<CanOpenUrlOptions> for String{
    fn into(self) -> CanOpenUrlOptions {
        CanOpenUrlOptions { url: self }
    }
}

impl<'a> Into<CanOpenUrlOptions> for &'a str{
    fn into(self) -> CanOpenUrlOptions {
        CanOpenUrlOptions { url: self.to_string() }
    }
}


#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct OpenUrlOptions {
    pub url: String,
}

impl Into<OpenUrlOptions> for String{
    fn into(self) -> OpenUrlOptions {
        OpenUrlOptions { url: self }
    }
}

impl<'a> Into<OpenUrlOptions> for &'a str{
    fn into(self) -> OpenUrlOptions {
        OpenUrlOptions { url: self.to_string() }
    }
}


#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CanOpenURLResult {

    pub value: bool,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct OpenURLResult {

    pub completed: bool,
}


pub struct AppLauncher;

impl AppLauncher {

    /// Check if an app can be opened with the given URL.
    pub async fn can_open_url(options: impl Into<CanOpenUrlOptions>)-> Result<CanOpenURLResult, Error>{
        run_value_value(options, can_open_url).await
    }

    /// Open an app with the given URL
    pub async fn open_url(options: impl Into<OpenUrlOptions>)-> Result<OpenURLResult, Error>{
        run_value_value(options, open_url).await
    }

}
