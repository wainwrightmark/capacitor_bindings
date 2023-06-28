use crate::extern_functions::*;
use serde::{Deserialize, Serialize};

use crate::helpers::*;
use crate::{error::Error};

pub struct Device;

impl Device {
    /// Return a unique identifier for the device.
    pub async fn get_id() -> Result<DeviceId, Error> {
        run_unit_value(device_get_id).await
    }

    /// Return information about the underlying device/os/platform.
    pub async fn get_info() -> Result<DeviceInfo, Error> {
        run_unit_value(device_get_info).await
    }

    /// Return information about the battery.
    pub async fn get_battery_info() -> Result<BatteryInfo, Error> {
        run_unit_value(device_get_battery_info).await
    }

    /// Get the device's current language locale code.
    pub async fn get_language_code() -> Result<GetLanguageCodeResult, Error> {
        run_unit_value(device_get_language_code).await
    }

    /// Get the device's current language locale tag.
    pub async fn get_language_tag() -> Result<LanguageTag, Error> {
        run_unit_value(device_get_language_tag).await
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceId {
    /// The identifier of the device as available to the app. This identifier may change on modern mobile platforms that only allow per-app install ids. On iOS, the identifier is a UUID that uniquely identifies a device to the app’s vendor (read more). on Android 8+, the identifier is a 64-bit number (expressed as a hexadecimal string), unique to each combination of app-signing key, user, and device (read more). On web, a random identifier is generated and stored on localStorage for subsequent calls. If localStorage is not available a new random identifier will be generated on every call.
    pub identifier: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    #[serde(default)]
    /// The name of the device. For example, "John's iPhone". This is only supported on iOS and Android 7.1 or above.
    pub name: Option<String>,
    #[serde(default)]
    /// The device model. For example, "iPhone13,4".
    pub model: String,
    /// The device platform (lowercase).
    pub platform: Platform,
    /// The operating system of the device.
    pub operating_system: OperatingSystem,
    #[serde(default)]
    /// The version of the device OS.
    pub os_version: String,
    #[serde(default)]
    /// The manufacturer of the device.
    pub manufacturer: String,
    #[serde(default)]
    /// Whether the app is running in a simulator/emulator.
    pub is_virtual: bool,
    #[serde(default)]
    /// Approximate memory used by the current app, in bytes. Divide by 1048576 to get the number of MBs used.
    pub mem_used: Option<u64>,
    #[serde(default)]
    /// How much free disk space is available on the normal data storage path for the os, in bytes. On Android it returns the free disk space on the "system" partition holding the core Android OS. On iOS this value is not accurate.
    pub disk_free: Option<u64>,
    #[serde(default)]
    /// The total size of the normal data storage path for the OS, in bytes. On Android it returns the disk space on the "system" partition holding the core Android OS.
    pub disk_total: Option<u64>,
    #[serde(default)]
    /// How much free disk space is available on the normal data storage, in bytes.
    pub real_disk_free: Option<u64>,
    #[serde(default)]
    /// The total size of the normal data storage path, in bytes.
    pub real_disk_total: Option<u64>,
    #[serde(default)]
    /// The web view browser version
    pub web_view_version: Option<String>,
    #[serde(rename = "iOSVersion")]
    #[serde(default)]
    /// The iOS version number. Only available on iOS. Multi-part version numbers are crushed down into an integer padded to two-digits, ex: "16.3.1" -> 160301
    pub ios_version: Option<String>,
    #[serde(rename = "androidSDKVersion	")]
    #[serde(default)]
    /// The Android SDK version number. Only available on Android.
    pub android_sdk_version: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct BatteryInfo {
    /// A percentage (0 to 1) indicating how much the battery is charged.
    pub battery_level: f64,
    /// Whether the device is charging.
    pub is_charging: bool,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GetLanguageCodeResult {
    /// Two character language code.
    pub value: String,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LanguageTag {
    /// Returns a well-formed IETF BCP 47 language tag.
    pub value: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    IOs,
    Android,
    Web,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperatingSystem {
    IOs,
    Android,
    Windows,
    Mac,
    #[default]
    Unknown,
}
