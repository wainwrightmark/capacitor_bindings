use crate::listener::*;
use capacitor_bindings::screen_reader::*;
use capacitor_bindings::helpers::Error;
use capacitor_bindings::helpers::PluginListenerHandle;
use log::info;
use std::future::Future;
use std::pin::Pin;
use yew::prelude::*;
use yewdux::store::Store;

#[cfg(any(feature = "android", feature= "ios"))]
listener_state!(
    StateChangeState,
    ScreenReader::add_state_change_listener,
    "Screen Reader Changed"
);

#[function_component(ScreenReaderView)]
pub fn screen_reader_view() -> Html {
    #[cfg(any(feature = "android", feature= "ios"))]
    {
        html! {
            <details>
            <summary>
                {"Screen Reader"}
                <span class="icon">{"↓"}</span>
            </summary>
            <br/>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_|crate::app::do_and_toast_result(ScreenReader::is_enabled)}> {"Is Enabled"}</button>
                <button onclick={|_|crate::app::do_async(|| {ScreenReader::speak("Hello World")})}> {"Speak"}</button>
                <ListenerButton<StateChangeState> />

            </div>
        </details>
        }
    }

    #[cfg(not(any(feature = "ios", feature = "android")))]
    {
        html! {
            <details>
                <summary>
                    {"Screen Reader"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <br/>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_|crate::app::do_and_toast_result(||{ScreenReader::speak("Hello World")})}> {"Speak"}</button>
                </div>
            </details>
        }
    }
}
