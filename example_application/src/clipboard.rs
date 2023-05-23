use crate::app::*;
use capacitor_bindings::clipboard::*;
use yew::prelude::*;

#[function_component(ClipboardView)]
pub fn clipboard_view() -> Html {
    html!(
        <details>
            <summary>
                {"Clipboard"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <br/>
                <button onclick={|_| do_and_toast_result(||Clipboard::write(WriteOptions::builder().string("Hello from Capacitor").build())) }> {"Set Clipboard"}</button>

                <button onclick={|_| do_and_toast_result(||Clipboard::read()) }> {"Read Clipboard"}</button>
            </div>
        </details>
    )
}
