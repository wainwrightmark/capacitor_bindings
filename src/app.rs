use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::extern_functions::*;
use crate::helpers::*;

pub struct App;

impl App {
    /// Force exit the app. This should only be used in conjunction with the backButton handler for Android to exit the app when navigation is complete.

    /// Ionic handles this itself so you shouldn't need to call this if using Ionic.
    pub async fn exit_app() -> Result<(), Error> {
        run_unit_unit(app_exit_app).await
    }

    /// Minimizes the application.

    /// Only available for Android.
    #[cfg(feature = "android")]
    pub async fn minimize_app() -> Result<(), Error> {
        run_unit_unit(app_minimize_app).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    /// Return information about the app.
    pub async fn get_info() -> Result<AppInfo, Error>{
        run_unit_value(app_get_info).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    /// Gets the current app state.
    pub async fn get_state() -> Result<AppState, Error>{
        run_unit_value(app_get_state).await
    }

    /// Get the URL the app was launched with, if any.
    pub async fn get_launch_url() -> Result<Option<AppLaunchUrl>, Error>{
        run_unit_value(app_get_launch_url).await
    }
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    /// The name of the app
    pub name: String,
    /// The identifier of the app. On iOS it's the Bundle Identifier. On Android it's the Application ID
    pub id: String,
    /// The build version. On iOS it's the CFBundleVersion. On Android it's the versionCode.
    pub build: String,

    /// The app version. On iOS it's the CFBundleShortVersionString. On Android it's package's versionName.
    pub version: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppState{
    /// Whether the app is active or not.
    pub is_active: bool
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppLaunchUrl{
    /// The url used to open the app.
    pub url: String
}
