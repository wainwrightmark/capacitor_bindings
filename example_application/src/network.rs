use capacitor_bindings::helpers::Error;
use capacitor_bindings::helpers::PluginListenerHandle;
use log::info;
use std::future::Future;
use std::pin::Pin;
use yew::prelude::*;

use capacitor_bindings::network::*;
use yewdux::store::Store;

use crate::listener::*;

#[derive(Debug, Default, Store, PartialEq, Clone)]
pub struct NetworkState {
    pub handle: Option<PluginListenerHandle>,
}

impl ListenerState for NetworkState {
    type Fut = Pin<Box<dyn Future<Output = Result<PluginListenerHandle, Error>>>>;

    fn get_handle(&self) -> &Option<PluginListenerHandle> {
        &self.handle
    }

    fn set_handle(&mut self, handle: Option<PluginListenerHandle>) {
        self.handle = handle
    }

    fn add_listener() -> Self::Fut {
        Box::pin(Network::add_network_change_listener(|status| {
            info!("Network Changed {:?}", status);
            crate::app::show_toast_or_panic(format!("Network Changed: {:?}", status))
        }))
    }

    fn name() -> &'static str {
        "Network Changed"
    }

    fn take_handle(&mut self)-> Option<PluginListenerHandle> {
        self.handle.take()
    }
}

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
                <button onclick={|_|crate::app::do_and_toast_result(Network::get_status)}> {"Get Status"}</button>
                <ListenerButton<NetworkState> />

            </div>
        </details>
    )
}
