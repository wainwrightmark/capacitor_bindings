use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{extern_functions::*, helpers::*};

pub struct Preferences;
impl Preferences {
    /// Configure the preferences plugin at runtime.
    pub async fn configure(options: impl Into<ConfigureOptions>) -> Result<(), Error> {
        run_value_unit(options, preferences_configure).await
    }

    /// Set the value in preferences for a given key.
    pub async fn set(options: impl Into<SetOptions>) -> Result<(), Error> {
        run_value_unit(options, preferences_set).await
    }

    /// Remove the value from preferences for a given key, if any.
    pub async fn remove(options: impl Into<RemoveOptions>) -> Result<(), Error> {
        run_value_unit(options, preferences_remove).await
    }

    /// Get the value from preferences of a given key.
    pub async fn get(options: impl Into<GetOptions>) -> Result<GetResult, Error> {
        run_value_value(options, preferences_get).await
    }

    /// Clear keys and values from preferences.
    pub async fn clear() -> Result<(), Error> {
        run_unit_unit(preferences_clear).await
    }

    /// Return the list of known keys in preferences.
    pub async fn keys() -> Result<KeysResult, Error> {
        run_unit_value(preferences_keys).await
    }
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ConfigureOptions {
    /// Set the preferences group. Preferences groups are used to organize key/value pairs.
    pub group: String,
}

impl From<&str> for ConfigureOptions {
    fn from(val: &str) -> Self {
        ConfigureOptions {
            group: val.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GetResult {
    /// The value from preferences associated with the given key.
    pub value: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GetOptions {
    /// The key whose value to retrieve from preferences.
    pub key: String,
}

impl From<&str> for GetOptions {
    fn from(val: &str) -> Self {
        GetOptions {
            key: val.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SetOptions {
    /// The key to associate with the value being set in preferences.
    pub key: String,
    /// The value to set in preferences with the associated key.
    pub value: String,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemoveOptions {
    /// The key whose value to remove from preferences.
    pub key: String,
}

impl From<&str> for RemoveOptions {
    fn from(val: &str) -> Self {
        RemoveOptions {
            key: val.to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct KeysResult {
    /// The known keys in preferences.
    pub keys: Vec<String>,
}
