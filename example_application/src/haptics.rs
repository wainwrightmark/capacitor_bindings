use crate::app::*;
use capacitor_bindings::haptics::*;
use yew::prelude::*;

#[function_component(HapticsView)]
pub fn haptics_view() -> Html {
    html!(
        <details>
            <summary>
                {"Haptics"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_| do_and_toast_result(||Haptics::vibrate(3000.)) }> {"Vibrate 3s"}</button>
                <button onclick={|_| do_and_toast_result(||Haptics::impact(ImpactStyle::Heavy))}> {"Impact Heavy"}</button>
                <button onclick={|_| do_and_toast_result(||Haptics::impact(ImpactStyle::Medium))}> {"Impact Medium"}</button>
                <button onclick={|_| do_and_toast_result(||Haptics::impact(ImpactStyle::Light))}> {"Impact Light"}</button>

                <button onclick={|_| do_and_toast_result(||Haptics::notification(NotificationType::Success))}> {"Success"}</button>
                <button onclick={|_| do_and_toast_result(||Haptics::notification(NotificationType::Warning))}> {"Warning"}</button>
                <button onclick={|_| do_and_toast_result(||Haptics::notification(NotificationType::Error ))}> {"Error"}</button>

                <button onclick={|_| do_and_toast_result(||Haptics::selection_start())}> {"Selection Start"}</button>
                <button onclick={|_| do_and_toast_result(||Haptics::selection_changed())}> {"Selection Changed"}</button>
                <button onclick={|_| do_and_toast_result(||Haptics::selection_end())}> {"Selection End"}</button>
            </div>
        </details>
    )
}
