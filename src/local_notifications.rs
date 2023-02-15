use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};

#[wasm_bindgen()]
extern "C" {
    /// Schedule one or more local notifications.
    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="schedule" )]
    async fn schedule(options: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="registerActionTypes" )]
    async fn register_action_types(options: JsValue);

    #[wasm_bindgen(js_namespace = ["Capacitor", "Plugins", "LocalNotifications"], js_name="addListener" )]
    async fn add_listener(eventName: &str, listener_func: &Closure<dyn Fn(JsValue)>) -> JsValue;
}

pub struct LocalNotifications;

impl LocalNotifications {
    /// Schedule one or more local notifications.
    pub async fn schedule(options: &ScheduleOptions) -> ScheduleResult {
        let js_input_val = serde_wasm_bindgen::to_value(options).unwrap();

        let js_output_val = schedule(js_input_val).await;

        let output = serde_wasm_bindgen::from_value(js_output_val).unwrap();
        output
    }

    pub async fn register_action_types(options: &RegisterActionTypesOptions) {
        let js_val = serde_wasm_bindgen::to_value(options).unwrap();
        register_action_types(js_val).await;
    }

    #[must_use]
    pub async fn add_received_listener<'a, F: Fn(LocalNotificationSchema) + 'static>(
        func: F,
    ) -> &'a PluginListenerHandle {
        let func2 = move |js_value: JsValue| {
            let schema: LocalNotificationSchema = serde_wasm_bindgen::from_value(js_value)
                .expect("Should be LocalNotificationSchema");
            func(schema)
        };
        let closure = Closure::new(func2);
        let _js_val = add_listener("localNotificationReceived", &closure).await;
        // handle_js_value.1
        &PluginListenerHandle {}
    }

    #[must_use]
    pub async fn add_action_performed_listener<'a, F: Fn(ActionPerformed) + 'static>(
        func: F,
    ) -> &'a PluginListenerHandle {
        let func2 = move |js_value: JsValue| {
            let action_performed: ActionPerformed =
                serde_wasm_bindgen::from_value(js_value).expect("Should be ActionPerformed");
            func(action_performed)
        };
        let closure = Closure::new(func2);
        let _js_val = add_listener("localNotificationActionPerformed", &closure).await;
        // handle_js_value.1
        &PluginListenerHandle {}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleOptions {
    pub notifications: Vec<LocalNotificationSchema>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RegisterActionTypesOptions {
    pub types: Vec<ActionType>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    /// The action identifier. Referenced in the 'actionPerformed' event as actionId.
    pub id: String,

    /// The title text to display for this action.
    pub title: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActionPerformed {
    /// The identifier of the performed action. This might be "tap" if the user tapped the notification.
    pub action_id: String,
    /// The value entered by the user on the notification. Only available on iOS for notifications with input set to true.
    pub input_value: Option<String>,
    /// The original notification schema.
    pub notification: LocalNotificationSchema,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PluginListenerHandle {
    // js_val: JsValue
    // remove: js_sys::Function
}

// impl Drop for PluginListenerHandle{
//     fn drop(&mut self) {
//         self.remove.call0(&JsValue::null());
//     }
// }

/// A collection of actions.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActionType {
    /// The ID of the action type. Referenced in notifications by the actionTypeId key.
    pub id: String,

    /// The list of actions associated with this action type.
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleResult {
    /// The list of scheduled notifications.
    pub notifications: Vec<LocalNotificationDescriptor>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalNotificationDescriptor {
    pub id: i32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalNotificationSchema {
    /// The title of the notification.
    pub title: String,
    /// The body of the notification, shown below the title.
    pub body: String,

    /// Schedule this notification for a later time.
    pub schedule: Schedule,

    /// Sets a multiline text block for display in a big text notification style.
    pub large_body: Option<String>,

    /// Used to set the summary text detail in inbox and big text notification styles. Only available for Android.
    pub summary_text: Option<String>,

    /// The notification identifier. On Android it's a 32-bit int. So the value should be between -2147483648 and 2147483647 inclusive.
    pub id: i32,

    /// If true, the notification can't be swiped away. Calls setOngoing() on NotificationCompat.Builder with the provided value. Only available for Android.
    pub ongoing: bool,

    /// If true, the notification is canceled when the user clicks on it. Calls setAutoCancel() on NotificationCompat.Builder with the provided value. Only available for Android.
    pub auto_cancel: bool,

    /// Sets a list of strings for display in an inbox style notification. Up to 5 strings are allowed. Only available for Android.
    pub inbox_list: Option<Vec<String>>,

    /// Set a custom status bar icon. If set, this overrides the smallIcon option from Capacitor configuration. Icons should be placed in your app's res/drawable folder. The value for this option should be the drawable resource ID, which is the filename without an extension. Only available for Android.
    pub small_icon: Option<String>,

    /// Set a large icon for notifications. Icons should be placed in your app's res/drawable folder. The value for this option should be the drawable resource ID, which is the filename without an extension. Only available for Android.
    pub large_icon: Option<String>,

    /// Set the color of the notification icon. Only available for Android.
    pub icon_color: Option<String>,

    /// Associate an action type with this notification.
    pub action_type_id: Option<String>,

    /// Used to group multiple notifications. Calls setGroup() on NotificationCompat.Builder with the provided value. Only available for Android.
    pub group: Option<String>,

    /// If true, this notification becomes the summary for a group of notifications. Calls setGroupSummary() on NotificationCompat.Builder with the provided value. Only available for Android when using group.
    pub group_summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    /// Allow this notification to fire while in Doze Only available for Android 23+. Note that these notifications can only fire once per 9 minutes, per app.
    pub allow_while_idle: bool,

    /// Schedule a notification on particular interval(s). This is similar to scheduling cron jobs. Only available for iOS and Android.
    pub on: ScheduleOn,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]

pub struct ScheduleOn {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
    pub weekday: Option<Weekday>,

    pub hour: Option<u32>,
    pub minute: Option<u32>,
    pub second: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum Weekday {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_test::assert_tokens;

    fn get_options() -> ScheduleOptions {
        ScheduleOptions {
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
                small_icon: None,
                large_icon: None,
                icon_color: None,
                action_type_id: None,
                group: None,
                group_summary: None,
            }],
        }
    }

    #[test]
    fn test_ser_json() {
        let options = get_options();

        let str: String = serde_json::to_string(&options).unwrap();

        let expected = "{\"notifications\":[{\"title\":\"Notification Title\",\"body\":\"Notification Body\",\"schedule\":{\"allowWhileIdle\":true,\"on\":{\"second\":0}},\"largeBody\":\"Notification Large Body\",\"summaryText\":\"Notification Summary Text\",\"id\":123,\"ongoing\":false,\"autoCancel\":true,\"inboxList\":[\"N One\",\"N Two\",\"N Three\",\"N Four\",\"N Five\"]}]}";

        assert_eq!(str.trim(), expected.trim())
    }

    #[test]
    fn test_ser_de() {
        let options = get_options();

        {
            use serde_test::Token::*;

            assert_tokens(
                &options,
                &[
                    Struct {
                        name: "ScheduleOptions",
                        len: 1,
                    },
                    Str("notifications"),
                    Seq {
                        len: Option::Some(1),
                    },
                    Struct {
                        name: "LocalNotificationSchema",
                        len: 9,
                    },
                    Str("title"),
                    Str("Notification Title"),
                    Str("body"),
                    Str("Notification Body"),
                    Str("schedule"),
                    Struct {
                        name: "Schedule",
                        len: 2,
                    },
                    Str("allowWhileIdle"),
                    Bool(true),
                    Str("on"),
                    Struct {
                        name: "ScheduleOn",
                        len: 1,
                    },
                    Str("second"),
                    Some,
                    U32(0).to_owned(),
                    StructEnd,
                    StructEnd,
                    Str("largeBody"),
                    Some,
                    Str("Notification Large Body"),
                    Str("summaryText"),
                    Some,
                    Str("Notification Summary Text"),
                    Str("id"),
                    I32(123),
                    Str("ongoing"),
                    Bool(false),
                    Str("autoCancel"),
                    Bool(true),
                    Str("inboxList"),
                    Some,
                    Seq {
                        len: Option::Some(5),
                    },
                    Str("N One"),
                    Str("N Two"),
                    Str("N Three"),
                    Str("N Four"),
                    Str("N Five"),
                    SeqEnd,
                    StructEnd,
                    SeqEnd,
                    StructEnd,
                ],
            )
        }
    }
}
