use crate::app::*;
use capacitor_bindings::toast::*;
use yew::prelude::*;

#[function_component(ToastView)]
pub fn toast_view() -> Html {
    html!(
        <details>
            <summary>
                {"Toast"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_| do_and_toast_result(||Toast::show(ShowOptions::builder().text("Top Position (bottom on Android/Web)".to_string()).position(ToastPosition::Top).build())) }> {"Top"}</button>
                <button onclick={|_| do_and_toast_result(||Toast::show(ShowOptions::builder().text("Center Position (bottom on Android/Web)".to_string()).position(ToastPosition::Center).build())) }> {"Center"}</button>
                <button onclick={|_| do_and_toast_result(||Toast::show(ShowOptions::builder().text("Bottom Position".to_string()).position(ToastPosition::Bottom).build())) }> {"Bottom"}</button>
                <button onclick={|_| do_and_toast_result(||Toast::show(ShowOptions::builder().text("Long Duration (3500ms)".to_string()).duration(ToastDuration::Long).build())) }> {"Long"}</button>
                <button onclick={|_| do_and_toast_result(||Toast::show(ShowOptions::builder().text("Short Duration (2000ms)".to_string()).duration(ToastDuration::Short).build())) }> {"Short"}</button>
            </div>
        </details>
    )
}
