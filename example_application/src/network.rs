use capacitor_bindings::error::Error;
use capacitor_bindings::plugin_listener_handle::PluginListenerHandle;
use log::info;
use std::future::Future;
use std::pin::Pin;
use yew::prelude::*;

use capacitor_bindings::network::*;
use yewdux::store::Store;

use crate::app::*;
use crate::listener::*;
pub use crate::listener_state;

listener_state!(
    NetworkState,
    Network::add_network_change_listener,
    "Network Changed"
);

#[function_component(NetworkView)]
pub fn network_view() -> Html {
    html!(
        <details>
            <summary>
                {"Network"}
                <span class="icon">{"↓"}</span>
            </summary>
            <br/>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_|do_and_toast_result(Network::get_status)}> {"Get Status"}</button>
                <ListenerButton<NetworkState> />

            </div>
        </details>
    )
}
