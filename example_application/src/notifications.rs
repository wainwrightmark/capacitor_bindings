use capacitor_bindings::error::Error;
use capacitor_bindings::local_notifications::*;
use capacitor_bindings::plugin_listener_handle::PluginListenerHandle;
use chrono::{Datelike, Duration, Timelike};
use core::time;
use log::info;
use std::future::Future;
use std::pin::Pin;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yewdux::store::Store;

use crate::app::do_and_toast_result;
use crate::listener::*;

listener_state!(
    NotificationState,
    LocalNotifications::add_received_listener,
    "Notification Received"
);
listener_state!(
    NotificationActionState,
    LocalNotifications::add_action_performed_listener,
    "Notification Action"
);

#[function_component(NotificationView)]
pub fn notification_view() -> Html {
    html!(
        <details>
            <summary>
                {"Local Notifications"}
                <span class="icon">{"↓"}</span>
            </summary>
            <br/>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_| register_action_types()}> {"Register Action Types"}</button>
                <ListenerButton<NotificationState> />
                <ListenerButton<NotificationActionState> />

                <button onclick={|_| schedule_notifications()}> {"Schedule Notifications"}</button>
                <button onclick={|_| schedule_a_notification()}> {"Schedule a notification"}</button>

                <button onclick={|_| crate::app::do_and_toast_result(LocalNotifications::are_enabled)}> {"Are Enabled"}</button>
                <button onclick={|_| crate::app::do_and_toast_result(LocalNotifications::get_delivered_notifications)}> {"Get Delivered Notifications"}</button>

                <button onclick={|_| crate::app::do_and_toast_result(LocalNotifications::remove_all_delivered_notifications)}> {"Remove All Delivered Notifications"}</button>

                <button onclick={|_| crate::app::do_and_toast_result(LocalNotifications::check_permissions)}> {"Check Permissions"}</button>
                <button onclick={|_| crate::app::do_and_toast_result(LocalNotifications::request_permissions)}> {"Request Permissions"}</button>

            </div>
        </details>

    )
}

fn schedule_notifications() {
    do_and_toast_result(|| {
        let options = LocalNotificationSchema::builder()
            .title("Notification Title 1")
            .body("Notification Body 1")
            .auto_cancel(true)
            .schedule(ScheduleOn::builder().second(0).build())
            .id(1)
            .large_body("Notification Large Body 1")
            .summary_text("Notification Summary Text 1")
            .inbox_list(vec![
                "N One".into(),
                "N Two".into(),
                "N Three".into(),
                "N Four".into(),
                "N Five".into(),
            ])
            .build();

        LocalNotifications::schedule(options)
    });
}

fn schedule_a_notification() {
    do_and_toast_result(|| {
        let time: chrono::DateTime<chrono::Utc> = chrono::Utc::now() + Duration::seconds(5); //notify 5 seconds from now
        let time_string = time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let at_millis = JsValue::from_f64(js_sys::Date::parse(&time_string));
        let at = js_sys::Date::new(&at_millis);

        if let Some(at_str) = at.to_utc_string().as_string() {
            info!("Scheduling notification at {at_str}")
        }

        let options = LocalNotificationSchema::builder()
            .title("Notification Title 2")
            .body("Notification Body 2")
            .auto_cancel(true)
            .schedule(Schedule::At {
                at,
                repeats: false,
                allow_while_idle: true,
            })
            .id(2)
            .large_body("Notification Large Body 2")
            .summary_text("Notification Summary Text 2")
            .inbox_list(vec![
                "N One".into(),
                "N Two".into(),
                "N Three".into(),
                "N Four".into(),
                "N Five".into(),
            ])
            .build();

        LocalNotifications::schedule(options)
    });
}

fn register_action_types() {
    #[cfg(any(feature = "android", feature = "ios"))]
    {
        do_and_toast_result(|| {
            let options = RegisterActionTypesOptions {
                types: vec![ActionType {
                    id: "MyActionType".to_string(),
                    actions: vec![
                        Action {
                            id: "Foo".to_string(),
                            title: "Foo".to_string(),
                        },
                        Action {
                            id: "Bar".to_string(),
                            title: "Bar".to_string(),
                        },
                    ],
                }],
            };
            LocalNotifications::register_action_types(options)
        });
    }
    #[cfg(not(any(feature = "android", feature = "ios")))]
    {
        crate::app::show_toast_or_panic("No need to register action types except on android or ios")
    }
}
