use yew::prelude::*;

use capacitor_bindings::app::App;

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

                </div>
            </details>
        }
    }
}
