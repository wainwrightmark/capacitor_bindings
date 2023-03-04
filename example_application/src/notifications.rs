use std::future::Future;
use std::pin::Pin;
use capacitor_bindings::helpers::Error;
use capacitor_bindings::helpers::PluginListenerHandle;
use capacitor_bindings::local_notifications::*;
use log::info;
use yew::prelude::*;
use yewdux::store::Store;

use crate::app::do_and_toast_result;
use crate::listener::*;


#[derive(Debug, Default, Store, PartialEq, Clone)]
pub struct NotificationState{
    pub handle: Option<PluginListenerHandle>,
}

#[derive(Debug, Default, Store, PartialEq, Clone)]
pub struct NotificationActionState{
    pub handle: Option<PluginListenerHandle>,
}

impl ListenerState for NotificationState{
    type Fut = Pin<Box<dyn Future<Output = Result<PluginListenerHandle, Error>>>>;

        fn get_handle(&self) -> &Option<PluginListenerHandle> {
            &self.handle
        }

        fn set_handle(&mut self, handle: Option<PluginListenerHandle>) {
            self.handle = handle
        }

        fn take_handle(&mut self)-> Option<PluginListenerHandle> {
            self.handle.take()
        }

        fn add_listener() -> Self::Fut {
            Box::pin(LocalNotifications::add_received_listener(|status| {
                info!("Notification: {:?}", status);
                crate::app::show_toast_or_panic(format!("Notification: {:?}", status))
            }))
        }

        fn name() -> &'static str {
            "Notification Received"
        }
}


impl ListenerState for NotificationActionState{
    type Fut = Pin<Box<dyn Future<Output = Result<PluginListenerHandle, Error>>>>;

        fn get_handle(&self) -> &Option<PluginListenerHandle> {
            &self.handle
        }

        fn set_handle(&mut self, handle: Option<PluginListenerHandle>) {
            self.handle = handle
        }

        fn take_handle(&mut self)-> Option<PluginListenerHandle> {
            self.handle.take()
        }

        fn add_listener() -> Self::Fut {
            Box::pin(LocalNotifications::add_action_performed_listener(|status| {
                info!("Action: {:?}", status);
                crate::app::show_toast_or_panic(format!("Action: {:?}", status))
            }))
        }

        fn name() -> &'static str {
            "Notification Action"
        }
}


pub enum NotificationAction{
    ListenNotifications, Remove
}

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
            </div>
        </details>

    )
}


fn schedule_notifications() {
    do_and_toast_result(|| {
        let options = ScheduleOptions {
            notifications: vec![LocalNotificationSchema {
                auto_cancel: true,
                body: "Notification Body".to_string(),
                title: "Notification Title".to_string(),
                schedule: Schedule {
                    on: ScheduleOn {
                        second: Some(0),
                        year: None,
                        minute: None,
                        month: None,
                        day: None,
                        weekday: None,
                        hour: None,
                    },
                    allow_while_idle: true,
                },
                large_body: Some("Notification Large Body".to_string()),
                summary_text: Some("Notification Summary Text".to_string()),
                id: 123,
                ongoing: false,
                inbox_list: Some(vec![
                    "N One".to_string(),
                    "N Two".to_string(),
                    "N Three".to_string(),
                    "N Four".to_string(),
                    "N Five".to_string(),
                ]),
                action_type_id: Some("MyActionType".to_string()),
                group: None,
                group_summary: None,
                small_icon: None,
                large_icon: None,
                icon_color: None,
            }],
        };

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