use crate::app::*;
use capacitor_bindings::splash_screen   ::*;
use yew::prelude::*;

#[function_component(SplashView)]
pub fn splash_view() -> Html {
    html!(
        <details>
            <summary>
                {"Splash"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_| do_and_toast_result(||SplashScreen::show(ShowOptions::default())) }> {"Show Splash"}</button>
            </div>
        </details>
    )
}
