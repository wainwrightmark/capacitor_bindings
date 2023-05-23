use capacitor_bindings::helpers::Error;
use capacitor_bindings::helpers::PluginListenerHandle;
use capacitor_bindings::motion::Motion;
use log::info;
use std::future::Future;
use std::pin::Pin;
use yew::prelude::*;
use yewdux::store::Store;


use crate::listener::*;
pub use crate::listener_state;

listener_state!(
    OrientationState,
    Motion::add_orientation_listener,
    "Orientation Changed"
);


listener_state!(
    AccelerationState,
    Motion::add_acceleration_listener,
    "Acceleration Changed"
);

#[function_component(MotionView)]
pub fn motion_view() -> Html {
    html!(
        <details>
            <summary>
                {"Motion"}
                <span class="icon">{"↓"}</span>
            </summary>
            <br/>
            <div style="display: flex; flex-direction: column;">
                <ListenerButton<OrientationState> />
                <ListenerButton<AccelerationState> />

            </div>
        </details>
    )
}
