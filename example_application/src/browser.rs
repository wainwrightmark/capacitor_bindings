use crate::app::*;
use capacitor_bindings::browser::*;
use yew::prelude::*;

#[function_component(BrowserView)]
pub fn browser_view() -> Html {
    html!(
        <details>
            <summary>
                {"Browser"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <br/>
                <button onclick={|_| do_and_toast_result(|| Browser::open(OpenOptions { url: "https://capacitorjs.com/".to_string(), ..Default::default()})) }> {"Open"}</button>
            </div>
        </details>
    )
}
