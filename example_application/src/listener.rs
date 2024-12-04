use capacitor_bindings::plugin_listener_handle::PluginListenerHandle;
use std::any::type_name;
use std::future::Future;
use yew::function_component;
use yew::html;
use yew::Html;
use yewdux::prelude::*;

use yewdux::store::Store;

pub trait ListenerState: Clone + PartialEq + Store + Default {
    type Fut: Future<Output = Result<PluginListenerHandle, capacitor_bindings::error::Error>>;

    fn get_handle(&self) -> &Option<PluginListenerHandle>;
    fn take_handle(&mut self) -> Option<PluginListenerHandle>;
    fn set_handle(&mut self, handle: Option<PluginListenerHandle>);

    fn is_listening(&self) -> bool {
        self.get_handle().is_some()
    }

    fn add_listener() -> Self::Fut;

    fn name() -> &'static str;
}

async fn listen_async<T: ListenerState>(dispatch: Dispatch<T>) -> () {
    let mut state = (dispatch.get().as_ref()).clone();

    if let Some(previous_handle) = state.take_handle() {
        let r = previous_handle.remove_async().await;

        if let Err(err) = r {
            log::error!("{}", err);
            crate::app::show_toast_or_panic(err.to_string());
        } else {
            let message = format!("{} listener removed", type_name::<T>());
            log::info!("{}", message);
            crate::app::show_toast_or_panic(message);
        }
    }

    let result = T::add_listener().await;

    match result {
        Ok(handle) => {
            let message = format!("{} listener added", type_name::<T>());
            log::info!("{}", message);
            crate::app::show_toast_or_panic(message);
            state.set_handle(Some(handle));
        }
        Err(err) => {
            crate::app::show_toast_or_panic(err.to_string());
        }
    }

    dispatch.set(state);
}

async fn remove_async<T: ListenerState>(dispatch: Dispatch<T>) -> () {
    let mut state = (dispatch.get()).as_ref().clone();

    if let Some(previous_handle) = state.take_handle() {
        let r = previous_handle.remove_async().await;

        if let Err(err) = r {
            log::error!("{}", err);
            crate::app::show_toast_or_panic(err.to_string());
        } else {
            let message = format!("{} listener removed", type_name::<T>());
            log::info!("{}", message);
            crate::app::show_toast_or_panic(message);
        }
    }

    dispatch.set(state);
}

#[function_component(ListenerButton)]
pub fn listener_button<T: ListenerState>() -> Html {
    let name = T::name();
    let remove_label = format!("Remove {name} listener");
    let add_label = format!("Add {name} listener");
    let (network_state, dispatch) = use_store::<T>();

    let click_listen = dispatch.future_callback(listen_async);
    let click_remove = dispatch.future_callback(remove_async);
    if network_state.is_listening() {
        html!(
            <button onclick={click_remove}> { remove_label}</button>
        )
    } else {
        html!(
            <button onclick={click_listen}> {add_label}</button>
        )
    }
}

#[macro_export]
macro_rules! listener_state {
    ($state_name:ident, $func:expr, $message:literal) => {
        #[derive(Debug, Default, Store, PartialEq, Clone)]
        pub struct $state_name {
            pub handle: Option<PluginListenerHandle>,
        }

        impl ListenerState for $state_name {
            type Fut = Pin<Box<dyn Future<Output = Result<PluginListenerHandle, Error>>>>;

            fn get_handle(&self) -> &Option<PluginListenerHandle> {
                &self.handle
            }

            fn set_handle(&mut self, handle: Option<PluginListenerHandle>) {
                self.handle = handle
            }

            fn take_handle(&mut self) -> Option<PluginListenerHandle> {
                self.handle.take()
            }

            fn add_listener() -> Self::Fut {
                Box::pin($func(|arg| {
                    info!("{}: {:?}", $message, arg);
                    $crate::app::show_toast_or_panic(format!("{}: {:?}", $message, arg))
                }))
            }

            fn name() -> &'static str {
                $message
            }
        }
    };
}
