use crate::extern_functions::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helpers::*;

pub struct Network;

impl Network {
    /// Query the current status of the network connection.
    pub async fn get_status() -> Result<ConnectionStatus, Error> {
        run_unit_value(network_get_status).await
    }

    pub async fn add_network_change_listener<F: Fn(ConnectionStatus) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "networkStatusChange", network_add_listener).await
    }
}

/// Represents the state and type of the network connection.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ConnectionStatus {
    /// Whether there is an active connection or not.
    pub connected: bool,
    /// The type of network connection currently in use. If there is no active network connection, connectionType will be 'none'.
    pub connection_type: ConnectionType,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self {
            connected: false,
            connection_type: ConnectionType::Unknown,
        }
    }
}

/// The type of network connection that a device might have.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ConnectionType {
    Wifi,
    Cellular,
    None,
    Unknown,
}
