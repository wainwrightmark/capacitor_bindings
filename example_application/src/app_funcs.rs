use yew::prelude::*;
use capacitor_bindings::helpers::Error;
use capacitor_bindings::helpers::PluginListenerHandle;
use capacitor_bindings::app::App;
use crate::listener::*;
use yewdux::store::Store;
use std::future::Future;
use std::pin::Pin;
use log::info;

listener_state!(StateChangeState, App::add_state_change_listener, "App state Changed");
listener_state!(PauseState, App::add_pause_listener, "App Paused");
listener_state!(ResumeState, App::add_resume_listener, "App Resumed");


listener_state!(UrlOpenState, App::add_app_url_open_listener, "Url opened");

#[cfg(any(feature = "android", feature= "ios"))]
listener_state!(RestoreState, App::add_app_restored_listener, "App Restored");
#[cfg(any(feature = "android",))]
listener_state!(BackButtonState, App::add_back_button_listener, "Back Button");


#[function_component(AppView)]
pub fn app_view() -> Html {
    #[cfg(any(feature = "android",))]
    {
        html! {
            <details>
            <summary>
                {"App"}
                <span class="icon">{"↓"}</span>
            </summary>
            <br/>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_|crate::app::do_and_toast_result(App::get_info)}> {"Get Info"}</button>
                <button onclick={|_|crate::app::do_and_toast_result(App::get_state)}> {"Get State"}</button>
                <button onclick={|_|crate::app::do_and_toast_result(App::get_launch_url)}> {"Get Launch Url"}</button>
                <button onclick={|_|crate::app::do_async(App::minimize_app)}> {"Minimize"}</button>
                <button onclick={|_|crate::app::do_async(App::exit_app)}> {"Exit"}</button>
                <ListenerButton<StateChangeState> />
                <ListenerButton<PauseState> />
                <ListenerButton<ResumeState> />
                <ListenerButton<UrlOpenState> />

                <ListenerButton<RestoreState> />
                <ListenerButton<BackButtonState> />

            </div>
        </details>
        }
    }
    #[cfg(all(any(feature = "ios"), not(any(feature = "android"))))]
    {
        html! {
            <details>
            <summary>
                {"App"}
                <span class="icon">{"↓"}</span>
            </summary>
            <br/>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_|crate::app::do_and_toast_result(App::get_info)}> {"Get Info"}</button>
                <button onclick={|_|crate::app::do_and_toast_result(App::get_state)}> {"Get State"}</button>
                <button onclick={|_|crate::app::do_and_toast_result(App::get_launch_url)}> {"Get Launch Url"}</button>
                <button onclick={|_|crate::app::do_async(App::exit_app)}> {"Exit"}</button>
                <ListenerButton<StateChangeState> />
                <ListenerButton<PauseState> />
                <ListenerButton<ResumeState> />
                <ListenerButton<UrlOpenState> />
            </div>
        </details>
        }
    }

    #[cfg(not(any(feature = "ios", feature = "android")))]
    {
        html!{
            <details>
                <summary>
                    {"App"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <br/>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_|crate::app::do_and_toast_result(App::get_launch_url)}> {"Get Launch Url"}</button>
                    <ListenerButton<StateChangeState> />
                        <ListenerButton<PauseState> />
                        <ListenerButton<ResumeState> />
                        <ListenerButton<UrlOpenState> />
                </div>
            </details>
        }
    }
}
