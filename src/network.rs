use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};

use crate::helpers::*;

#[wasm_bindgen()]
extern "C" {
    /// Query the current status of the network connection.
    #[wasm_bindgen(catch,js_namespace = ["Capacitor", "Plugins", "Network"], js_name="getStatus" )]
    async fn get_status() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "Network"], js_name="addListener" )]
    fn add_listener_network(
        eventName: &str,
        listener_func: &Closure<dyn Fn(JsValue)>,
    )-> JsValue;
}

pub struct Network;

impl Network {
    /// Query the current status of the network connection.
    pub async fn get_status() -> Result<ConnectionStatus, Error> {
        run_unit_value(get_status).await
    }

    pub async fn add_network_change_listener<F: Fn(ConnectionStatus) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "networkStatusChange", add_listener_network).await
    }
}

/// Represents the state and type of the network connection.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatus {
    /// Whether there is an active connection or not.
    pub connected: bool,
    /// The type of network connection currently in use. If there is no active network connection, connectionType will be 'none'.
    pub connection_type: ConnectionType,
}

/// The type of network connection that a device might have.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ConnectionType {
    Wifi,
    Cellular,
    None,
    Unknown,
}
