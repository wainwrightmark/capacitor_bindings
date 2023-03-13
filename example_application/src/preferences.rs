use crate::app::*;
use capacitor_bindings::preferences::*;
use yew::prelude::*;

#[function_component(PreferencesView)]
pub fn preferences_view() -> Html {
    html!(
        <details>
            <summary>
                {"Preferences"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <br/>
                <button onclick={|_| do_and_toast_result(||Preferences::set(SetOptions{
                    key: "key1".to_string(),
                    value: "alpha".to_string()
                })) }> {"Set Alpha"}</button>

                <button onclick={|_| do_and_toast_result(||Preferences::set(SetOptions{
                    key: "key1".to_string(),
                    value: "beta".to_string()
                })) }> {"Set Beta"}</button>

                <button onclick={|_| do_and_toast_result(||Preferences::get("key1")) }> {"Get"}</button>

                <button onclick={|_| do_and_toast_result(||Preferences::remove("key1")) }> {"Remove"}</button>

                <button onclick={|_| do_and_toast_result(||Preferences::clear()) }> {"Clear"}</button>

                <button onclick={|_| do_and_toast_result(||Preferences::keys()) }> {"Keys"}</button>
            </div>
        </details>
    )
}
