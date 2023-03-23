use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::NoneAsEmptyString;

use crate::extern_functions::*;
use crate::helpers::*;

pub struct App;

impl App {
    /// Force exit the app. This should only be used in conjunction with the backButton handler for Android to exit the app when navigation is complete.
    ///
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
    pub async fn get_info() -> Result<AppInfo, Error> {
        run_unit_value(app_get_info).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    /// Gets the current app state.
    pub async fn get_state() -> Result<AppState, Error> {
        run_unit_value(app_get_state).await
    }

    /// Get the URL the app was launched with, if any.
    pub async fn get_launch_url() -> Result<Option<AppLaunchUrl>, Error> {
        run_unit_value(app_get_launch_url).await
    }

    /// Listen for changes in the app or the activity states.
    ///
    /// On iOS it's fired when the native UIApplication.willResignActiveNotification and UIApplication.didBecomeActiveNotification events get fired.
    /// On Android it's fired when the Capacitor's Activity onResume and onStop methods gets called.
    /// On Web it's fired when the document's visibilitychange gets fired.
    pub async fn add_state_change_listener<F: Fn(AppState) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "appStateChange", app_add_listener).await
    }

    /// Listen for when the app or the activity are paused.
    ///
    /// On iOS it's fired when the native UIApplication.didEnterBackgroundNotification event gets fired.
    /// On Android it's fired when the Capacitor's Activity onPause method gets called.
    /// On Web it's fired when the document's visibilitychange gets fired and document.hidden is true.
    pub async fn add_pause_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "pause", app_add_listener).await
    }

    /// Listen for when the app or activity are resumed.
    ///
    /// On iOS it's fired when the native UIApplication.willEnterForegroundNotification event gets fired.
    /// On Android it's fired when the Capacitor's Activity onResume method gets called, but only after resume has fired first.
    /// On Web it's fired when the document's visibilitychange gets fired and document.hidden is false.
    pub async fn add_resume_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "resume", app_add_listener).await
    }

    /// Listen for url open events for the app.
    /// This handles both custom URL scheme links as well as URLs your app handles (Universal Links on iOS and App Links on Android)
    pub async fn add_app_url_open_listener<F: Fn(URLOpenListenerEvent) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "appUrlOpen", app_add_listener).await
    }

    /// If the app was launched with previously persisted plugin call data, such as on Android when an activity returns to an app that was closed, this call will return any data the app was launched with, converted into the form of a result from a plugin call.
    ///
    /// On Android, due to memory constraints on low-end devices, it's possible that, if your app launches a new activity, your app will be terminated by the operating system in order to reduce memory consumption.
    ///
    /// For example, that means the Camera API, which launches a new Activity to take a photo, may not be able to return data back to your app.
    ///
    /// To avoid this, Capacitor stores all restored activity results on launch. You should add a listener for appRestoredResult in order to handle any plugin call results that were delivered when your app was not running.
    ///
    /// Once you have that result (if any), you can update the UI to restore a logical experience for the user, such as navigating or selecting the proper tab.
    ///
    /// We recommend every Android app using plugins that rely on external Activities (for example, Camera) to have this event and process handled.
    #[cfg(any(feature = "android"))]
    pub async fn add_app_restored_listener<F: Fn(RestoredListenerEvent) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "appRestoredResult", app_add_listener).await
    }

    /// Listen for the hardware back button event (Android only). Listening for this event will disable the default back button behaviour, so you might want to call window.history.back() manually. If you want to close the app, call App.exitApp().
    #[cfg(any(feature = "android"))]
    pub async fn add_back_button_listener<F: Fn(BackButtonListenerEvent) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "backButton", app_add_listener).await
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
pub struct AppState {
    /// Whether the app is active or not.
    pub is_active: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppLaunchUrl {
    /// The url used to open the app.
    pub url: String,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct URLOpenListenerEvent {
    /// The URL the app was opened with.
    pub url: String,

    /// The source application opening the app (iOS only) https://developer.apple.com/documentation/uikit/uiapplicationopenurloptionskey/1623128-sourceapplication
    pub ios_source_application: Option<String>, //todo check

    #[serde_as(as = "NoneAsEmptyString")]
    /// Whether the app should open the passed document in-place or must copy it first. https://developer.apple.com/documentation/uikit/uiapplicationopenurloptionskey/1623123-openinplace
    pub ios_open_in_place: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RestoredListenerEvent {
    /// The pluginId this result corresponds to. For example, Camera.
    pub plugin_id: String,

    /// The methodName this result corresponds to. For example, getPhoto
    pub method_name: String,

    /// The result data passed from the plugin. This would be the result you'd expect from normally calling the plugin method. For example, CameraPhoto
    pub data: String, // TODO check this

    /// Boolean indicating if the plugin call succeeded.
    pub success: bool,

    /// If the plugin call didn't succeed, it will contain the error message.
    pub error: Option<InnerError>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BackButtonListenerEvent {
    /// Indicates whether the browser can go back in history. False when the history stack is on the first entry.
    pub can_go_back: bool,
}
