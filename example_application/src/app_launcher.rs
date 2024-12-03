use crate::app::*;
use capacitor_bindings::app_launcher::AppLauncher;
use yew::prelude::*;

#[function_component(AppLauncherView)]
pub fn app_launcher_state() -> Html {
    html!(
        <details>
            <summary>
                {"App Launcher"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_| do_and_toast_result(||AppLauncher::can_open_url("com.numbersalad.app")) }> {"Can Open"}</button>
                <button onclick={|_| do_and_toast_result(||AppLauncher::open_url("com.numbersalad.app"))}> {"Open Url"}</button>
            </div>
        </details>
    )
}
