use crate::extern_functions::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use crate::helpers::*;
use crate::{error::Error, plugin_listener_handle::PluginListenerHandle};

pub struct LocalNotifications;

impl LocalNotifications {
    /// Check if notifications are enabled or not.
    pub async fn are_enabled() -> Result<EnabledResult, Error> {
        run_unit_value(local_notifications_are_enabled).await
    }

    /// Check permission to display local notifications.
    pub async fn check_permissions() -> Result<PermissionStatus, Error> {
        run_unit_value(local_notifications_check_permissions).await
    }

    /// Request permission to display local notifications.
    pub async fn request_permissions() -> Result<PermissionStatus, Error> {
        run_unit_value(local_notifications_request_permissions).await
    }

    /// Get a list of notifications that are visible on the notifications screen.
    pub async fn get_delivered_notifications() -> Result<DeliveredNotifications, Error> {
        run_unit_value(local_notifications_get_delivered_notifications).await
    }

    /// Remove the specified notifications from the notifications screen.
    pub async fn remove_delivered_notifications(
        delivered: impl Into<DeliveredNotifications>,
    ) -> Result<(), Error> {
        run_value_unit(
            delivered,
            local_notifications_remove_delivered_notifications,
        )
        .await
    }

    /// Cancel pending notifications.
    pub async fn cancel(options: impl Into<CancelOptions>) -> Result<(), Error> {
        run_value_unit(options, local_notifications_cancel).await
    }

    /// Remove all the notifications from the notifications screen.
    pub async fn remove_all_delivered_notifications() -> Result<(), Error> {
        run_unit_unit(local_notifications_remove_all_delivered_notifications).await
    }

    /// Schedule one or more local notifications.
    pub async fn schedule(options: impl Into<ScheduleOptions>) -> Result<ScheduleResult, Error> {
        run_value_value(options, local_notifications_schedule).await
    }

    /// Register actions to take when notifications are displayed.
    /// Only available for iOS and Android.
    #[cfg(any(feature = "ios", feature = "android"))]
    pub async fn register_action_types(
        options: impl Into<RegisterActionTypesOptions>,
    ) -> Result<(), Error> {
        run_value_unit(options, local_notifications_register_action_types).await
    }

    pub async fn add_received_listener<F: Fn(LocalNotificationSchema) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(
            func,
            "localNotificationReceived",
            local_notifications_add_listener,
        )
        .await
    }

    pub async fn add_action_performed_listener<F: Fn(ActionPerformed) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(
            func,
            "localNotificationActionPerformed",
            local_notifications_add_listener,
        )
        .await
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ScheduleOptions {
    pub notifications: Vec<LocalNotificationSchema>,
}

impl From<LocalNotificationSchema> for ScheduleOptions {
    fn from(val: LocalNotificationSchema) -> Self {
        ScheduleOptions {
            notifications: vec![val],
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RegisterActionTypesOptions {
    pub types: Vec<ActionType>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Action {
    /// The action identifier. Referenced in the 'actionPerformed' event as actionId.
    pub id: String,

    /// The title text to display for this action.
    pub title: String,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EnabledResult {
    /// Whether or not the device has local notifications enabled.
    pub value: bool,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    /// Whether or not the device has local notifications enabled.
    pub display: PermissionState,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ActionPerformed {
    /// The identifier of the performed action. This might be "tap" if the user tapped the notification.
    pub action_id: String,
    /// The value entered by the user on the notification. Only available on iOS for notifications with input set to true.
    pub input_value: Option<String>,
    /// The original notification schema.
    pub notification: LocalNotificationSchema,
}

impl ActionPerformed {
    pub fn is_tap(&self) -> bool {
        self.action_id == "tap"
    }
}

/// A collection of actions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ActionType {
    /// The ID of the action type. Referenced in notifications by the actionTypeId key.
    pub id: String,

    /// The list of actions associated with this action type.
    pub actions: Vec<Action>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ScheduleResult {
    /// The list of scheduled notifications.
    pub notifications: Vec<LocalNotificationDescriptor>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CancelOptions {
    /// The list of notifications to cancel.
    pub notifications: Vec<LocalNotificationDescriptor>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LocalNotificationDescriptor {
    pub id: i32,
}

impl Default for LocalNotificationSchema {
    fn default() -> Self {
        Self {
            title: Default::default(),
            body: Default::default(),
            schedule: Schedule::On {
                on: ScheduleOn::default(),
                allow_while_idle: false,
            },
            large_body: Default::default(),
            summary_text: Default::default(),
            id: Default::default(),
            ongoing: Default::default(),
            auto_cancel: Default::default(),
            inbox_list: Default::default(),
            small_icon: Default::default(),
            large_icon: Default::default(),
            icon_color: Default::default(),
            action_type_id: Default::default(),
            group: Default::default(),
            group_summary: Default::default(),
            sound: Default::default(),
            thread_identifier: Default::default(),
            summary_argument: Default::default(),
            channel_id: Default::default(),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase", default)]
pub struct LocalNotificationSchema {
    #[builder(setter(into))]
    /// The title of the notification.
    pub title: String,
    #[builder(setter(into))]
    /// The body of the notification, shown below the title.
    pub body: String,
    #[builder(setter(into))]
    /// Schedule this notification for a later time.
    pub schedule: Schedule,
    #[builder(setter(into, strip_option), default)]
    /// Sets a multiline text block for display in a big text notification style.
    pub large_body: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Used to set the summary text detail in inbox and big text notification styles. Only available for Android.
    pub summary_text: Option<String>,
    /// The notification identifier. On Android it's a 32-bit int. So the value should be between -2147483648 and 2147483647 inclusive.
    pub id: i32,
    #[builder(default)]
    /// If true, the notification can't be swiped away. Calls setOngoing() on NotificationCompat.Builder with the provided value. Only available for Android.
    pub ongoing: bool,
    /// If true, the notification is canceled when the user clicks on it. Calls setAutoCancel() on NotificationCompat.Builder with the provided value. Only available for Android.
    pub auto_cancel: bool,
    #[builder(setter(into, strip_option), default)]
    /// Sets a list of strings for display in an inbox style notification. Up to 5 strings are allowed. Only available for Android.
    pub inbox_list: Option<Vec<String>>,
    #[builder(setter(into, strip_option), default)]
    /// Set a custom status bar icon. If set, this overrides the smallIcon option from Capacitor configuration. Icons should be placed in your app's res/drawable folder. The value for this option should be the drawable resource ID, which is the filename without an extension. Only available for Android.
    pub small_icon: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Set a large icon for notifications. Icons should be placed in your app's res/drawable folder. The value for this option should be the drawable resource ID, which is the filename without an extension. Only available for Android.
    pub large_icon: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Set the color of the notification icon. Only available for Android.
    pub icon_color: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Associate an action type with this notification.
    pub action_type_id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Used to group multiple notifications. Calls setGroup() on NotificationCompat.Builder with the provided value. Only available for Android.
    pub group: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// If true, this notification becomes the summary for a group of notifications. Calls setGroupSummary() on NotificationCompat.Builder with the provided value. Only available for Android when using group.
    pub group_summary: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Name of the audio file to play when this notification is displayed. Include the file extension with the filename. On iOS, the file should be in the app bundle. On Android, the file should be in res/raw folder. Recommended format is .wav because is supported by both iOS and Android. Only available for iOS and Android < 26. For Android 26+ use channelId of a channel configured with the desired sound. If the sound file is not found, (i.e. empty string or wrong name) the default system notification sound will be used. If not provided, it will produce the default sound on Android and no sound on iOS.
    pub sound: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Used to group multiple notifications. Sets threadIdentifier on the UNMutableNotificationContent. Only available for iOS.
    pub thread_identifier: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// The string this notification adds to the category's summary format string. Sets summaryArgument on the UNMutableNotificationContent. Only available for iOS.
    pub summary_argument: Option<String>,
    #[builder(setter(into, strip_option), default)]
    /// Specifies the channel the notification should be delivered on. If channel with the given name does not exist then the notification will not fire. If not provided, it will use the default channel. Calls setChannelId() on NotificationCompat.Builder with the provided value. Only available for Android 26+.
    pub channel_id: Option<String>,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Schedule {
    On {
        /// Schedule a notification on particular interval(s). This is similar to scheduling cron jobs. Only available for iOS and Android.
        on: ScheduleOn,
        #[serde(rename = "allowWhileIdle")]
        /// Allow this notification to fire while in Doze Only available for Android 23+. Note that these notifications can only fire once per 9 minutes, per app.
        allow_while_idle: bool,
    },
    At {
        #[serde(
            skip_deserializing,
            default = "js_sys::Date::new_0",
            serialize_with = "serde_wasm_bindgen::preserve::serialize"
        )]


        /// WARNING: This will not be deserialized correctly
        ///
        /// Schedule a notification at a specific date and time.
        /// ```ignore
        /// let time: chrono::DateTime<chrono::Utc> = chrono::Utc::now() + Duration::seconds(5); //notify 5 seconds from now
        /// let time_string = time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        /// let at_millis = JsValue::from_f64(js_sys::Date::parse(&time_string));
        /// let at = js_sys::Date::new(&at_millis);
        /// ```
        at: js_sys::Date,

        /// Repeat delivery of this notification at the date and time specified by at. Only available for iOS and Android.
        repeats: bool,
        /// Allow this notification to fire while in Doze Only available for Android 23+. Note that these notifications can only fire once per 9 minutes, per app.
        allow_while_idle: bool,
    },
    Every {
        /// Schedule a notification on a particular interval.
        every: ScheduleEvery,
        /// Limit the number times a notification is delivered by the interval specified by every.
        count: usize,
        #[serde(rename = "allowWhileIdle")]
        /// Allow this notification to fire while in Doze Only available for Android 23+. Note that these notifications can only fire once per 9 minutes, per app.
        allow_while_idle: bool,
    },
}

impl From<ScheduleOn> for Schedule {
    fn from(val: ScheduleOn) -> Self {
        Schedule::On {
            on: val,
            allow_while_idle: true,
        }
    }
}

impl From<(ScheduleEvery, usize)> for Schedule {
    fn from(val: (ScheduleEvery, usize)) -> Self {
        Schedule::Every {
            every: val.0,
            count: val.1,
            allow_while_idle: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScheduleEvery {
    Year,
    Month,
    TwoWeeks,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase", default)]
pub struct ScheduleOn {
    #[builder(setter(strip_option), default)]
    pub year: Option<u32>,
    #[builder(setter(strip_option), default)]
    pub month: Option<u32>,
    #[builder(setter(strip_option), default)]
    pub day: Option<u32>,
    #[builder(setter(strip_option), default)]
    pub weekday: Option<Weekday>,
    #[builder(setter(strip_option), default)]
    pub hour: Option<u32>,
    #[builder(setter(strip_option), default)]
    pub minute: Option<u32>,
    #[builder(setter(strip_option), default)]
    pub second: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
pub enum PermissionState {
    Prompt,
    PromptWithRationale,
    Granted,
    Denied,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DeliveredNotifications {
    /// List of notifications that are visible on the notifications screen.
    pub notifications: Vec<DeliveredNotificationSchema>,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DeliveredNotificationSchema {
    /// The notification identifier.
    pub id: i32,
    /// The notification tag. Only available on Android.
    pub tag: Option<String>,
    /// The title of the notification.
    pub title: String,
    /// The body of the notification, shown below the title.
    pub body: String,
    /// The configured group of the notification. Only available for Android.
    pub group: Option<String>,
    /// If this notification is the summary for a group of notifications. Only available for Android.
    pub group_summary: Option<bool>,
    //pub data: Any,
    //pub extra: Any,
    //TODO additional fields
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_test::assert_tokens;

    fn get_options() -> ScheduleOptions {
        LocalNotificationSchema::builder()
            .title("Notification Title")
            .body("Notification Body")
            .auto_cancel(true)
            .schedule(ScheduleOn::builder().second(0).build())
            .id(123)
            .large_body("Notification Large Body")
            .summary_text("Notification Summary Text")
            .inbox_list(vec![
                "N One".into(),
                "N Two".into(),
                "N Three".into(),
                "N Four".into(),
                "N Five".into(),
            ])
            .build()
            .into()
    }

    #[test]
    fn test_ser_json() {
        let options = get_options();

        let str: String = serde_json::to_string(&options).unwrap();

        let expected = "{\"notifications\":[{\"title\":\"Notification Title\",\"body\":\"Notification Body\",\"schedule\":{\"on\":{\"second\":0},\"allowWhileIdle\":true},\"largeBody\":\"Notification Large Body\",\"summaryText\":\"Notification Summary Text\",\"id\":123,\"ongoing\":false,\"autoCancel\":true,\"inboxList\":[\"N One\",\"N Two\",\"N Three\",\"N Four\",\"N Five\"]}]}";

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
                    Str("on"),
                    Struct {
                        name: "ScheduleOn",
                        len: 1,
                    },
                    Str("second"),
                    Some,
                    U32(0).to_owned(),
                    StructEnd,
                    Str("allowWhileIdle"),
                    Bool(true),
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
