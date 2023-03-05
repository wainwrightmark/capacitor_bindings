use std::any::type_name;
use std::future::Future;
use std::rc::Rc;

use capacitor_bindings::helpers::PluginListenerHandle;
use yew::function_component;
use yew::html;
use yew::Html;
use yewdux::prelude::*;
use yewdux::store::AsyncReducer;
use yewdux::store::Store;

pub trait ListenerState: Clone + PartialEq + Store + Default {
    type Fut: Future<Output = Result<PluginListenerHandle, capacitor_bindings::helpers::Error>>;

    fn get_handle(&self) -> &Option<PluginListenerHandle>;
    fn take_handle(&mut self)-> Option<PluginListenerHandle>;
    fn set_handle(&mut self, handle: Option<PluginListenerHandle>);



    fn is_listening(&self) -> bool {
        self.get_handle().is_some()
    }

    fn add_listener() -> Self::Fut;

    fn name() -> &'static str;
}

pub enum NetworkAction {
    Listen,
    Remove,
}

#[async_reducer]
impl<T: ListenerState> AsyncReducer<T> for NetworkAction {
    async fn apply(self, state: Rc<T>) -> Rc<T> {
        let mut state = (state.as_ref()).clone();

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

        match self {
            NetworkAction::Listen => {
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
            }
            NetworkAction::Remove => {}
        }

        state.into()
    }
}

#[function_component(ListenerButton)]
pub fn listener_button<T: ListenerState>() -> Html {
    let name = T::name();
    let remove_label = format!("Remove {name} listener");
    let add_label = format!("Add {name} listener");
    let (network_state, dispatch) = use_store::<T>();

    let click_listen = dispatch.apply_future_callback(|_| NetworkAction::Listen);
    let click_remove = dispatch.apply_future_callback(|_| NetworkAction::Remove);
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
                    crate::app::show_toast_or_panic(format!("{}: {:?}", $message, arg))
                }))
            }

            fn name() -> &'static str {
                $message
            }


        }
    };
}