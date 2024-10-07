use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins","SplashScreen"], js_name="show" )]
    pub(crate) async fn splash_show(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins","SplashScreen"], js_name="hide" )]
    pub(crate) async fn splash_hide(options: JsValue) -> Result<(), JsValue>;
}

#[wasm_bindgen()]
extern "C" {

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins","Clipboard"], js_name="write" )]
    pub(crate) async fn write(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch,final, js_namespace = ["Capacitor", "Plugins","Clipboard"],js_name="read" )]
    pub(crate) async fn read() -> Result<JsValue, JsValue>;

}

#[wasm_bindgen()]
extern "C" {

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins","Dialog"], js_name="alert" )]
    pub(crate) async fn alert(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch,final, js_namespace = ["Capacitor", "Plugins","Dialog"],js_name="prompt" )]
    pub(crate) async fn prompt(options: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins","Dialog"],js_name="confirm" )]
    pub(crate) async fn confirm(options: JsValue) -> Result<JsValue, JsValue>;

}

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "ScreenReader"], js_name = "isEnabled")]
    pub(crate) async fn screen_reader_is_enabled() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "ScreenReader"], js_name = "speak")]
    pub(crate) async fn screen_reader_speak(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen( final,js_namespace = ["Capacitor", "Plugins", "ScreenReader"], js_name="addListener" )]
    pub(crate) fn screen_reader_add_listener(
        eventName: &str,
        listener_func: &Closure<dyn Fn(JsValue)>,
    ) -> JsValue;
}

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "ActionSheet"], js_name = "showActions")]
    pub(crate) async fn action_sheet_show_actions(options: JsValue) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen()]
extern "C" {

    /// Prompt the user to pick a photo from an album, or take a new photo with the camera.
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Camera"], js_name = "getPhoto")]
    pub(crate) async fn camera_get_photo(options: JsValue) -> Result<JsValue, JsValue>;

    /// Allows the user to pick multiple pictures from the photo gallery. On iOS 13 and older it only allows to pick one picture.
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Camera"], js_name = "pickImages")]
    pub(crate) async fn camera_pick_images(options: JsValue) -> Result<JsValue, JsValue>;

    /// iOS 14+ Only: Allows the user to update their limited photo library selection. On iOS 15+ returns all the limited photos after the picker dismissal. On iOS 14 or if the user gave full access to the photos it returns an empty array.
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Camera"],js_name = "pickLimitedLibraryPhotos")]
    pub(crate) async fn camera_pick_limited_library_photos() -> Result<JsValue, JsValue>;

    /// iOS 14+ Only: Return an array of photos selected from the limited photo library.
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Camera"], js_name = "getLimitedLibraryPhotos")]
    pub(crate) async fn camera_get_limited_library_photos() -> Result<JsValue, JsValue>;

    /// Check camera and photo album permissions
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Camera"], js_name = "checkPermissions")]
    pub(crate) async fn camera_check_permissions() -> Result<JsValue, JsValue>;

    /// Request camera and photo album permissions. Not implemented on Web
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Camera"], js_name = "requestPermissions")]
    pub(crate) async fn camera_request_permissions(options: JsValue) -> Result<JsValue, JsValue>;

}

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Device"], js_name="getId" )]
    pub(crate) async fn device_get_id() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Device"], js_name="getInfo" )]
    pub(crate) async fn device_get_info() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Device"], js_name="getBatteryInfo" )]
    pub(crate) async fn device_get_battery_info() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Device"], js_name="getLanguageCode" )]
    pub(crate) async fn device_get_language_code() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Device"], js_name="getLanguageTag" )]
    pub(crate) async fn device_get_language_tag() -> Result<JsValue, JsValue>;

}

#[wasm_bindgen()]
extern "C" {

    /// Vibrate the device
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Haptics"], js_name="vibrate")]
    pub(crate) async fn haptics_vibrate(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Haptics"], js_name="impact")]
    pub(crate) async fn haptics_impact(options: JsValue) -> Result<(), JsValue>;

    /// Trigger a haptics "notification" feedback
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Haptics"], js_name="notification")]
    pub(crate) async fn haptics_notification(options: JsValue) -> Result<(), JsValue>;

    /// Trigger a selection started haptic hint
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Haptics"], js_name="selectionStart")]
    pub(crate) async fn haptics_selectionStart() -> Result<(), JsValue>;

    /// Trigger a selection changed haptic hint. If a selection was started already, this will cause the device to provide haptic feedback
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Haptics"], js_name="selectionChanged")]
    pub(crate) async fn haptics_selectionChanged() -> Result<(), JsValue>;

    /// If selectionStart() was called, selectionEnd() ends the selection. For example, call this when a user has lifted their finger from a control
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Haptics"], js_name="selectionEnd")]
    pub(crate) async fn haptics_selectionEnd() -> Result<(), JsValue>;
}

#[wasm_bindgen()]
extern "C" {

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="areEnabled" )]
    pub(crate) async fn local_notifications_are_enabled() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="checkPermissions" )]
    pub(crate) async fn local_notifications_check_permissions() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="requestPermissions" )]
    pub(crate) async fn local_notifications_request_permissions() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="getDeliveredNotifications" )]
    pub(crate) async fn local_notifications_get_delivered_notifications() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="cancel" )]
    pub(crate) async fn local_notifications_cancel(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="removeDeliveredNotifications" )]
    pub(crate) async fn local_notifications_remove_delivered_notifications(
        delivered: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="removeAllDeliveredNotifications" )]
    pub(crate) async fn local_notifications_remove_all_delivered_notifications(
    ) -> Result<(), JsValue>;

    /// Schedule one or more local notifications.
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="schedule" )]
    pub(crate) async fn local_notifications_schedule(options: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="registerActionTypes" )]
    pub(crate) async fn local_notifications_register_action_types(
        options: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen( final,js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="addListener" )]
    pub(crate) fn local_notifications_add_listener(
        eventName: &str,
        listener_func: &Closure<dyn Fn(JsValue)>,
    ) -> JsValue;
}

#[wasm_bindgen()]
extern "C" {
    /// Query the current status of the network connection.
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Network"], js_name="getStatus" )]
    pub(crate) async fn network_get_status() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(final,js_namespace = ["Capacitor", "Plugins", "Network"], js_name="addListener" )]
    pub(crate) fn network_add_listener(
        eventName: &str,
        listener_func: &Closure<dyn Fn(JsValue)>,
    ) -> JsValue;
}

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Share"], js_name="canShare" )]
    pub(crate) async fn share_can_share() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Share"], js_name="share" )]
    pub(crate) async fn share_share(options: JsValue) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="show" )]
    pub(crate) async fn status_bar_show() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="hide" )]
    pub(crate) async fn status_bar_hide() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="setStyle" )]
    pub(crate) async fn status_bar_set_style(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="setBackgroundColor" )]
    pub(crate) async fn status_bar_set_background_color(options: JsValue) -> Result<(), JsValue>;

    /// Set whether or not the status bar should overlay the webview to allow usage of the space underneath it.
    /// This method is only supported on Android.
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "StatusBar"], js_name="setOverlaysWebView" )]
    pub(crate) async fn status_bar_set_overlays_web_view(options: JsValue) -> Result<(), JsValue>;
}

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Toast"], js_name="show" )]
    pub(crate) async fn toast_show(options: JsValue) -> Result<(), JsValue>;
}

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "App"], js_name="exitApp" )]
    pub(crate) async fn app_exit_app() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "App"], js_name="minimizeApp" )]
    pub(crate) async fn app_minimize_app() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "App"], js_name="getInfo" )]
    pub(crate) async fn app_get_info() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "App"], js_name="getState" )]
    pub(crate) async fn app_get_state() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "App"], js_name="getLaunchUrl" )]
    pub(crate) async fn app_get_launch_url() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(final, js_namespace = ["Capacitor", "Plugins", "App"], js_name="addListener" )]
    pub(crate) fn app_add_listener(
        eventName: &str,
        listener_func: &Closure<dyn Fn(JsValue)>,
    ) -> JsValue;
}

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Preferences"], js_name="configure" )]
    pub(crate) async fn preferences_configure(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Preferences"], js_name="set" )]
    pub(crate) async fn preferences_set(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Preferences"], js_name="remove" )]
    pub(crate) async fn preferences_remove(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Preferences"], js_name="get" )]
    pub(crate) async fn preferences_get(options: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Preferences"], js_name="clear" )]
    pub(crate) async fn preferences_clear() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Preferences"], js_name="keys" )]
    pub(crate) async fn preferences_keys() -> Result<JsValue, JsValue>;
}

#[wasm_bindgen()]
extern "C" {

    /// Open a page with the specified options.
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "Browser"], js_name = "open")]
    pub(crate) async fn browser_open(options: JsValue) -> Result<(), JsValue>;

    /// Web & iOS only: Close an open browser window. No-op on other platforms.
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Browser"], js_name = "close")]
    pub(crate) async fn browser_close() -> Result<(), JsValue>;

    #[wasm_bindgen(final, js_namespace = ["Capacitor", "Plugins", "Browser"],js_name = "addListener")]
    pub(crate) fn browser_add_listener(
        eventName: &str,
        listener_func: &Closure<dyn Fn(JsValue)>,
    ) -> JsValue;

    /// Remove all native listeners for this plugin.
    #[wasm_bindgen(catch, final,js_namespace = ["Capacitor", "Plugins", "Browser"], js_name = "removeAllListeners")]
    pub(crate) async fn browser_remove_all_listeners() -> Result<(), JsValue>;
}

#[cfg(all(feature = "review_plugin"))]
#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "InAppReview"], js_name="requestReview" )]
    pub(crate) async fn request_review() -> Result<(), JsValue>;
}

#[cfg(feature = "game_plugin")]
#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "CapacitorGameConnect"], js_name="signIn" )]
    pub(crate) async fn game_connect_sign_in() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "CapacitorGameConnect"], js_name="showLeaderboard" )]
    pub(crate) async fn game_connect_show_leaderboard(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "CapacitorGameConnect"], js_name="submitScore" )]
    pub(crate) async fn game_connect_submit_score(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "CapacitorGameConnect"], js_name="showAchievements" )]
    pub(crate) async fn game_connect_show_achievements() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "CapacitorGameConnect"], js_name="unlockAchievement" )]
    pub(crate) async fn game_connect_unlock_achievement(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "CapacitorGameConnect"], js_name="incrementAchievementProgress" )]
    pub(crate) async fn game_connect_increment_achievement_progress(
        options: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "CapacitorGameConnect"], js_name="getUserTotalScore" )]
    pub(crate) async fn game_connect_get_user_total_score(
        options: JsValue,
    ) -> Result<JsValue, JsValue>;

}

#[cfg(feature = "admob_plugin")]
#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="initialize" )]
    pub(crate) async fn admob_initialize(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="trackingAuthorizationStatus" )]
    pub(crate) async fn admob_tracking_authorization_status() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="requestTrackingAuthorization" )]
    pub(crate) async fn admob_request_tracking_authorization() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="setApplicationMuted" )]
    pub(crate) async fn admob_set_application_muted(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="setApplicationVolume" )]
    pub(crate) async fn admob_set_application_volume(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="showBanner" )]
    pub(crate) async fn admob_show_banner(options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="hideBanner" )]
    pub(crate) async fn admob_hide_banner() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="resumeBanner" )]
    pub(crate) async fn admob_resume_banner() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="removeBanner" )]
    pub(crate) async fn admob_remove_banner() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="requestConsentInfo" )]
    pub(crate) async fn admob_request_consent_info(options: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="showConsentForm" )]
    pub(crate) async fn admob_show_consent_form() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="resetConsentInfo" )]
    pub(crate) async fn admob_reset_consent_info() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="prepareInterstitial" )]
    pub(crate) async fn admob_prepare_interstitial(options: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="showInterstitial" )]
    pub(crate) async fn admob_show_interstitial() -> Result<(), JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="prepareRewardVideoAd" )]
    pub(crate) async fn admob_prepare_reward_video_ad(options: JsValue)
        -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="showRewardVideoAd" )]
    pub(crate) async fn admob_show_reward_video_ad() -> Result<JsValue, JsValue>;

    #[wasm_bindgen( final,js_namespace = ["Capacitor", "Plugins", "AdMob"], js_name="addListener" )]
    pub(crate) fn admob_add_listener(
        eventName: &str,
        listener_func: &Closure<dyn Fn(JsValue)>,
    ) -> JsValue;
}
