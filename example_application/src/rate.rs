use crate::app::*;
use capacitor_bindings::rate::*;
use yew::prelude::*;

#[function_component(RateView)]
pub fn rate_view() -> Html {
    #[cfg(any(feature = "ios", feature = "android"))]
    {
        html!(
            <details>
                <summary>
                    {"Rate"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_| do_and_toast_result(||Rate::request_review()) }> {"Request Review"}</button>
                </div>
            </details>
        )
    }
    #[cfg(not(any(feature = "ios", feature = "android")))]
    {
        html!(<></>)
    }
}
