#[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
use crate::{extern_functions::*, prelude::*};
use serde::{Deserialize, Serialize};
use serde_repr::*;
use typed_builder::TypedBuilder;

/// For details, https://github.com/openforge/capacitor-game-connect
pub struct GameConnect {}

impl GameConnect {
    /// Initialize AdMob with AdMobInitializationOptions
    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    pub async fn initialize(options: impl Into<AdMobInitializationOptions>) -> Result<(), Error> {
        run_value_unit(options, admob_initialize).await
    }

    /// Confirm requestTrackingAuthorization status (iOS >14)
    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    pub async fn tracking_authorization_status() -> Result<TrackingAuthorizationStatus, Error> {
        run_unit_value(admob_tracking_authorization_status).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// request requestTrackingAuthorization (iOS >14). This is deprecated method. We recommend UMP Consent.
    #[deprecated]
    pub async fn request_tracking_authorization() -> Result<(), Error> {
        run_unit_unit(admob_request_tracking_authorization).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Report application mute state to AdMob SDK
    pub async fn set_application_muted(
        options: impl Into<ApplicationMutedOptions>,
    ) -> Result<(), Error> {
        run_value_unit(options, admob_set_application_muted).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Report application volume to AdMob SDK
    pub async fn set_application_volume(
        options: impl Into<ApplicationVolumeOptions>,
    ) -> Result<(), Error> {
        run_value_unit(options, admob_set_application_volume).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Show a banner Ad
    pub async fn show_banner(options: impl Into<BannerAdOptions>) -> Result<(), Error> {
        run_value_unit(options, admob_show_banner).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Hide the banner, remove it from screen, but can show it later
    pub async fn hide_banner() -> Result<(), Error> {
        run_unit_unit(admob_hide_banner).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Resume the banner, show it after hide
    pub async fn resume_banner() -> Result<(), Error> {
        run_unit_unit(admob_resume_banner).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    pub async fn add_banner_ad_sized_changed_listener<F: Fn(AdMobBannerSize) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "bannerAdSizeChanged", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    pub async fn add_banner_ad_loaded_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "bannerAdLoaded", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    pub async fn add_banner_failed_to_load_listener<F: Fn(AdMobError) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "bannerAdFailedToLoad", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    pub async fn add_banner_opened_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "bannerAdOpened", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    pub async fn add_banner_closed_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "bannerAdClosed", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    pub async fn add_banner_ad_impression_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "bannerAdImpression", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Request user consent information
    pub async fn request_consent_info(
        options: impl Into<AdmobConsentRequestOptions>,
    ) -> Result<AdmobConsentInfo, Error> {
        run_value_value(options, admob_request_consent_info).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Shows a google user consent form (rendered from your GDPR message config).
    pub async fn show_consent_form() -> Result<AdmobConsentInfo, Error> {
        run_unit_value(admob_show_consent_form).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Resets the UMP SDK state. Call requestConsentInfo function again to allow user modify their consent
    pub async fn reset_consent_info() -> Result<(), Error> {
        run_unit_unit(admob_reset_consent_info).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Prepare interstitial banner
    pub async fn prepare_interstitial(options: impl Into<AdOptions>) -> Result<AdLoadInfo, Error> {
        run_value_value(options, admob_prepare_interstitial).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Show interstitial ad when it’s ready
    pub async fn show_interstitial() -> Result<(), Error> {
        run_unit_unit(admob_show_interstitial).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits after trying to prepare and Interstitial, when it is loaded and ready to be show
    pub async fn add_interstitial_ad_loaded_listener<F: Fn(AdLoadInfo) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "interstitialAdLoaded", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits after trying to prepare and Interstitial, when it could not be loaded
    pub async fn add_interstitial_failed_to_load_listener<F: Fn(AdMobError) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "interstitialAdFailedToLoad", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits when the Interstitial ad is visible to the user
    pub async fn add_interstitial_showed_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "interstitialAdShowed", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits when the Interstitial ad is failed to show
    pub async fn add_interstitial_ad_failed_to_show_listener<F: Fn(AdMobError) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "interstitialAdFailedToShow", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits when the Interstitial ad is not visible to the user anymore.
    pub async fn add_interstitial_ad_dismissed_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "interstitialAdDismissed", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Prepare a reward video ad
    pub async fn prepare_reward_video_ad(
        options: impl Into<RewardAdOptions>,
    ) -> Result<AdLoadInfo, Error> {
        run_value_value(options, admob_prepare_reward_video_ad).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Show a reward video ad
    pub async fn show_reward_video_ad() -> Result<AdMobRewardItem, Error> {
        run_unit_value(admob_show_reward_video_ad).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits after trying to prepare a RewardAd when it could not be loaded
    pub async fn add_reward_failed_to_load_listener<F: Fn(AdMobError) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "onRewardedVideoAdFailedToLoad", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits after trying to prepare a RewardAd and the Video is loaded and ready to be show
    pub async fn add_reward_ad_loaded_listener<F: Fn(AdLoadInfo) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "onRewardedVideoAdLoaded", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits when user get rewarded from AdReward
    pub async fn add_reward_ad_rewarded_listener<F: Fn(AdMobRewardItem) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "onRewardedVideoAdReward", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits when the AdReward video is not visible to the user anymore. Important: This has nothing to do with the reward it self. This event will emits in this two cases: 1. The user starts the video ad but close it before the reward emit. 2. The user start the video and see it until end, then gets the reward and after that the ad is closed.
    pub async fn add_reward_ad_dismissed_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "onRewardedVideoAdDismissed", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits when the AdReward video is failed to show
    pub async fn add_reward_ad_failed_to_show_listener<F: Fn(AdMobError) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "onRewardedVideoAdFailedToShow", admob_add_listener).await
    }

    #[cfg(all(feature = "admob_plugin", any(feature = "ios", feature = "android")))]
    /// Emits when the AdReward video is visible to the user
    pub async fn add_reward_showed_listener<F: Fn(()) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "onRewardedVideoAdShowed", admob_add_listener).await
    }
}

#[derive(Clone, Debug, Serialize, TypedBuilder)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct AdMobInitializationOptions {
    ///An Array of devices IDs that will be marked as tested devices if `AdMobInitializationOptions.initializeForTesting` is true (Real Ads will be served to Testing devices, but they will not count as 'real').
    #[builder(setter(into))]
    #[serde(rename = "testingDevices")]
    pub testing_devices: Vec<String>,

    /// If set to true, the devices on `AdMobInitializationOptions.testingDevices` will be registered to receive test production ads.
    #[builder(setter(into))]
    #[serde(rename = "initializeForTesting")]
    pub initialize_for_testing: bool,

    /// For purposes of the Children's Online Privacy Protection Act (COPPA), there is a setting called tagForChildDirectedTreatment.
    #[builder(setter(into))]
    #[serde(rename = "tagForChildDirectedTreatment")]
    pub tag_for_child_directed_treatment: bool,

    /// When using this feature, a Tag For Users under the Age of Consent in Europe (TFUA) parameter will be included in all future ad requests.
    #[builder(setter(into))]
    #[serde(rename = "tagForUnderAgeOfConsent")]
    pub tag_for_under_age_of_consent: bool,

    /// As an app developer, you can indicate whether you want Google to treat your content as child-directed when you make an ad request.
    #[builder(setter(into))]
    #[serde(rename = "maxAdContentRating")]
    pub max_ad_content_rating: MaxAdContentRating,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum MaxAdContentRating {
    /// Content suitable for general audiences, including families.
    General,
    /// Content suitable for most audiences with parental guidance.
    ParentalGuidance,
    /// Content suitable for teen and older audiences.
    Teen,
    /// Content suitable only for mature audiences.
    MatureAudience,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingAuthorizationStatusInterface {
    pub status: TrackingAuthorizationStatus,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TrackingAuthorizationStatus {
    Authorized,
    Denied,
    NotDetermined,
    Restricted,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationMutedOptions {
    /// To inform the SDK that the app volume has been muted. Note: Video ads that are ineligible to be shown with muted audio are not returned for ad requests made, when the app volume is reported as muted or set to a value of 0. This may restrict a subset of the broader video ads pool from serving.
    pub muted: bool,
}

impl From<bool> for ApplicationMutedOptions {
    fn from(value: bool) -> Self {
        Self { muted: value }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationVolumeOptions {
    /// If your app has its own volume controls (such as custom music or sound effect volumes), disclosing app volume to the Google Mobile Ads SDK allows video ads to respect app volume settings. enable set 0.0 - 1.0, any float allowed.
    pub volume: f32,
}

impl From<f32> for ApplicationVolumeOptions {
    fn from(value: f32) -> Self {
        Self {
            volume: value.clamp(0.0, 1.0),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// When notice listener of OnAdLoaded, you can get banner size.
pub struct AdMobBannerSize {
    pub width: f32,
    pub height: f32,
}

/// For more information https://developers.google.com/android/reference/com/google/android/gms/ads/AdError
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)] //TODO remove default
#[serde(rename_all = "camelCase")]
pub struct AdMobError {
    /// Gets the error's code.
    pub code: u32,
    /// Gets the message describing the error.
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdmobConsentInfo {
    /// The consent status of the user.
    pub status: AdmobConsentStatus,
    /// If `true` a consent form is available and vice versa.
    pub is_consent_form_available: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdmobConsentRequestOptions {
    /// Sets the debug geography to test the consent locally.
    pub debug_geography: AdmobConsentDebugGeography,
    /// An array of test device IDs to allow. Note: On iOS, the ID may renew if you uninstall and reinstall the app.
    pub test_device_identifiers: Vec<String>,
    /// Set to true to provide the option for the user to accept being shown personalized ads.
    pub tag_for_under_age_of_consent: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TypedBuilder, Default)] //todo remove default
#[serde(rename_all = "camelCase")]
pub struct AdLoadInfo {
    pub ad_unit_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdOptions {
    /// The ad unit ID that you want to request
    pub ad_id: String,
    /// You can use test mode of ad.
    pub is_testing: bool,
    /// Margin Banner. Default is 0px; If position is BOTTOM_CENTER, margin is be margin-bottom. If position is TOP_CENTER, margin is be margin-top.
    pub margin: f32,
    /// The default behavior of the Google Mobile Ads SDK is to serve personalized ads. Set this to true to request Non-Personalized Ads
    pub npa: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TypedBuilder, Default)] //todo REMOVE default

pub struct AdMobRewardItem {
    /// Rewarded type user got
    #[serde(rename = "type")]
    pub reward_type: String,
    /// Rewarded amount user got
    #[serde(rename = "amount")]
    pub amount: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdmobConsentStatus {
    /// User consent not required.
    NotRequired,
    /// User consent already obtained.
    Obtained,
    /// User consent required but not yet obtained.
    Required,
    /// Unknown consent status, AdsConsent.requestInfoUpdate needs to be called to update it.
    Unknown,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum AdmobConsentDebugGeography {
    /// Debug geography disabled.
    Disabled = 0,
    /// Geography appears as in EEA for debug devices.
    EEA = 1,
    /// Geography appears as not in EEA for debug devices.
    NotEea = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]

pub enum InterstitialAdPluginEvents {
    /// Emits after trying to prepare and Interstitial, when it is loaded and ready to be show
    #[serde(rename = "interstitialAdLoaded")]
    Loaded,
    /// Emits after trying to prepare and Interstitial, when it could not be loaded
    #[serde(rename = "interstitialAdFailedToLoad")]
    FailedToLoad,
    /// Emits when the Interstitial ad is visible to the user
    #[serde(rename = "interstitialAdShowed")]
    Showed,
    /// Emits when the Interstitial ad is failed to show
    #[serde(rename = "interstitialAdFailedToShow")]
    FailedToShow,
    /// Emits when the Interstitial ad is not visible to the user anymore.
    #[serde(rename = "interstitialAdDismissed")]
    Dismissed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RewardAdPluginEvents {
    /// Emits after trying to prepare a RewardAd and the Video is loaded and ready to be show
    #[serde(rename = "onRewardedVideoAdLoaded")]
    Loaded,
    /// Emits after trying to prepare a RewardAd when it could not be loaded
    #[serde(rename = "onRewardedVideoAdFailedToLoad")]
    FailedToLoad,
    /// Emits when the AdReward video is visible to the user
    #[serde(rename = "onRewardedVideoAdShowed")]
    Showed,
    /// Emits when the AdReward video is failed to show
    #[serde(rename = "onRewardedVideoAdFailedToShow")]
    FailedToShow,
    /// Emits when the AdReward video is not visible to the user anymore. Important: This has nothing to do with the reward it self. This event will emits in this two cases: 1. The user starts the video ad but close it before the reward emit. 2. The user start the video and see it until end, then gets the reward and after that the ad is closed.
    #[serde(rename = "onRewardedVideoAdDismissed")]
    Dismissed,
    /// Emits when user get rewarded from AdReward
    #[serde(rename = "onRewardedVideoAdReward")]
    Rewarded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BannerAdSize {
    /// Mobile Marketing Association (MMA) banner ad size (320x50 density-independent pixels).
    Banner,
    /// Interactive Advertising Bureau (IAB) full banner ad size (468x60 density-independent pixels).
    FullBanner,
    /// Large banner ad size (320x100 density-independent pixels).
    LargeBanner,
    /// Interactive Advertising Bureau (IAB) medium rectangle ad size (300x250 density-independent pixels).
    MediumRectangle,
    /// Interactive Advertising Bureau (IAB) leaderboard ad size (728x90 density-independent pixels).
    Leaderboard,
    /// A dynamically sized banner that is full-width and auto-height.
    AdaptiveBanner,
    SmartBanner,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BannerAdPosition {
    /// Banner position be top-center
    TopCenter,
    /// Banner position be center
    Center,
    /// Banner position be bottom-center(default)
    BottomCenter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BannerAdPluginEvents {
    SizeChanged,
    Loaded,
    FailedToLoad,
    /// Open "Adsense" Event after user click banner
    Opened,
    /// Close "Adsense" Event after user click banner
    Closed,
    /// Similarly, this method should be called when an impression is recorded for the ad by the mediated SDK.
    AdImpression,
}

/// If you have enabled SSV in your AdMob Application. You can provide customData or a userId be passed to your callback to do further processing on. Important You HAVE to define one of them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RewardAdSSV {
    pub user_id: Option<String>,
    pub custom_data: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RewardAdOptions {
    /// If you have enabled SSV in your AdMob Application. You can provide customData or a userId be passed to your callback to do further processing on. Important You HAVE to define one of them.
    pub ssv: Option<RewardAdSSV>,

    /// The ad unit ID that you want to request
    pub ad_id: String,
    /// You can use test mode of ad.
    pub is_testing: bool,
    /// Margin Banner. Default is 0px; If position is BOTTOM_CENTER, margin is be margin-bottom. If position is TOP_CENTER, margin is be margin-top.
    pub margin: f32,
    /// The default behavior of the Google Mobile Ads SDK is to serve personalized ads. Set this to true to request Non-Personalized Ads
    pub npa: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BannerAdOptions {
    pub ad_size: BannerAdSize,
    pub position: BannerAdPosition,

    /// The ad unit ID that you want to request
    pub ad_id: String,
    /// You can use test mode of ad.
    pub is_testing: bool,
    /// Margin Banner. Default is 0px; If position is BOTTOM_CENTER, margin is margin-bottom. If position is TOP_CENTER, margin is margin-top.
    pub margin: f32,
    /// The default behavior of the Google Mobile Ads SDK is to serve personalized ads. Set this to true to request Non-Personalized Ads
    pub npa: bool,
}
