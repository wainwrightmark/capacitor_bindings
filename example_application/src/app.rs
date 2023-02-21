use std::fmt::Debug;
use std::future::Future;

use capacitor_bindings::action_sheet::*;
use capacitor_bindings::local_notifications::*;

use capacitor_bindings::toast;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use capacitor_bindings::camera::*;
use capacitor_bindings::device::*;
use capacitor_bindings::dialog::*;
use capacitor_bindings::haptics::*;
use capacitor_bindings::share::*;
use capacitor_bindings::status_bar::*;
use capacitor_bindings::toast::*;

#[function_component(App)]
pub fn app() -> Html {
    let register_action_types = |_: MouseEvent| {
        #[cfg(any(feature = "android", feature = "ios"))]
        {
            async fn register() {
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
                LocalNotifications::register_action_types(options).await
            }

            spawn_local(register());
        }
        #[cfg(not(any(feature = "android", feature = "ios")))]
        {
            spawn_local(Toast::show(
                "No need to register action types except on android or ios",
            ))
        }
    };

    let schedule_notifications = |_: MouseEvent| {
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

        wasm_bindgen_futures::spawn_local(schedule_notification(options));
    };

    let listen_for_notifications = |_: MouseEvent| {
        wasm_bindgen_futures::spawn_local(add_local_notification_received_listener());
    };

    let listen_for_notification_actions = |_: MouseEvent| {
        wasm_bindgen_futures::spawn_local(add_action_performed_listener());
    };

    let show_actions_onclick = |_: MouseEvent| {
        wasm_bindgen_futures::spawn_local(show_actions());
    };

    let status_block: Html;
    #[cfg(any(feature = "android", feature = "ios"))]
    {
        status_block = html! {
            <details>
            <summary>
                {"Status Bar"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_| spawn_local(StatusBar::show())}> {"Show Status"}</button>
                <button onclick={|_| spawn_local(StatusBar::hide())}> {"Hide Status"}</button>
                <button onclick={|_| spawn_local(StatusBar::set_style(Style::Light))}> {"Set Status Light"}</button>
                <button onclick={|_| spawn_local(StatusBar::set_style(Style::Dark))}> {"Set Status Dark"}</button>
                <button onclick={|_| spawn_local(StatusBar::set_background_color("#22DD44"))}> {"Set Status Background Green"}</button>
            </div>
            </details>
        };
    };

    #[cfg(not(any(feature = "android", feature = "ios")))]
    {
        status_block = html! {
            <details>
            <summary>
                {"Status Bar"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <button>
                    {"Cannot set status bar without android or ios feature"}
                </button>
            </div>
            </details>
        };
    };

    html! {
        <main>
            <div style="display: flex; flex-direction: column;">
            <button onclick={|_| spawn_local(Toast::show("Hello Toast"))}> {"Show Toast"}</button>

            <button onclick={show_actions_onclick}> {"Show Actions"}</button>

            <details>
                <summary>
                    {"Share"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_|spawn_local(can_share())}> {"Can Share?"}</button>
                    <button onclick={|_|spawn_local(do_share())}> {"Share"}</button>
                </div>
            </details>

            <details>
                <summary>
                    {"Camera"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_|do_and_toast_result(get_photo)}> {"Get Photo"}</button>
                    <button onclick={|_|do_and_toast_result(check_permissions)}> {"Check Permissions"}</button>
                    <button onclick={|_|do_and_toast_result(request_permissions)}> {"Request Permissions"}</button>
                </div>
            </details>

            {status_block}

            <details>
                <summary>
                    {"Dialog"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_|do_and_toast_result(||Dialog::alert(
                        AlertOptions{
                            button_title: "button title".to_string(),
                            message: "message".to_string(),
                            title: "title".to_string()

                        }))}> {"Alert"}</button>

                        <button onclick={|_|do_and_toast_result(||Dialog::prompt(
                        PromptOptions{
                            ok_button_title: "ok".to_string(),
                            cancel_button_title: "cancel".to_string(),
                            message: "message".to_string(),
                            title: "title".to_string(),
                            input_placeholder: Some("input placeholder".to_string()),
                            input_text: Some("input text".to_string()),

                        }))}> {"Prompt"}</button>

                        <button onclick={|_|do_and_toast_result(||Dialog::confirm(
                        ConfirmOptions{
                            ok_button_title: "ok".to_string(),
                            cancel_button_title: "cancel".to_string(),
                            message: "message".to_string(),
                            title: "title".to_string()

                        }))}> {"Confirm"}</button>
                </div>
            </details>

            <details>
                <summary>
                    {"Haptics"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_| spawn_local(Haptics::vibrate(3000.)) }> {"Vibrate 3s"}</button>
                    <button onclick={|_| spawn_local(Haptics::impact(ImpactStyle::Heavy))}> {"Impact Heavy"}</button>
                    <button onclick={|_| spawn_local(Haptics::impact(ImpactStyle::Medium))}> {"Impact Medium"}</button>
                    <button onclick={|_| spawn_local(Haptics::impact(ImpactStyle::Light))}> {"Impact Light"}</button>

                    <button onclick={|_| spawn_local(Haptics::notification(NotificationType::Success))}> {"Success"}</button>
                    <button onclick={|_| spawn_local(Haptics::notification(NotificationType::Warning))}> {"Warning"}</button>
                    <button onclick={|_| spawn_local(Haptics::notification(NotificationType::Error ))}> {"Error"}</button>


                    <button onclick={|_| spawn_local(Haptics::selection_start())}> {"Selection Start"}</button>
                    <button onclick={|_| spawn_local(Haptics::selection_changed())}> {"Selection Changed"}</button>
                    <button onclick={|_| spawn_local(Haptics::selection_end())}> {"Selection End"}</button>
                </div>
            </details>





            <details>
                <summary>
                    {"Device"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_| do_and_toast_result(||Device::get_id())}> {"Device Id"}</button>
                    <button onclick={|_| do_and_toast_result(||Device::get_info())}> {"Device Info"}</button>
                    <button onclick={|_| do_and_toast_result(||Device::get_battery_info())}> {"Battery Info"}</button>
                    <button onclick={|_| do_and_toast_result(||Device::get_language_code())}> {"Language Code"}</button>
                    <button onclick={|_| do_and_toast_result(||Device::get_language_tag())}> {"Language Tag"}</button>
                </div>
            </details>

            <details>
                <summary>
                    {"Local Notifications"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={register_action_types}> {"Register Action Types"}</button>
                    <button onclick={listen_for_notifications}> {"Listen For Notifications"}</button>
                    <button onclick={listen_for_notification_actions}> {"Listen For Notification Actions"}</button>
                    <button onclick={schedule_notifications}> {"Schedule Notifications"}</button>
                </div>
            </details>
            </div>
        </main>
    }
}

fn do_and_toast_result<T: Debug, Fut: Future<Output = T>, F: Fn() -> Fut + 'static>(f: F) {
    spawn_local(async move {
        let r = f().await;
        log::info!("{r:?}");
        Toast::show(format!("{r:?}")).await
    })
}

async fn check_permissions() -> PermissionStatus {
    Camera::check_permissions().await
}

async fn request_permissions() -> Option<PermissionStatus> {
    #[cfg(any(feature = "android", feature = "ios"))]
    {
        let ps = Camera::request_permissions(CameraPluginPermissions {
            permissions: vec![CameraPermissionType::Camera, CameraPermissionType::Photos],
        })
        .await;
        Some(ps)
    }
    #[cfg(not(any(feature = "android", feature = "ios")))]
    {
        spawn_local(Toast::show(
            "Cannot request permissions except on android or ios",
        ));
        None
    }
}

async fn get_photo() -> Photo {
    Camera::get_photo(ImageOptions {
        quality: 100,
        allow_editing: true,
        result_type: CameraResultType::Base64,
        save_to_gallery: false,
        width: 32,
        height: 32,
        correct_orientation: true,
        source: CameraSource::Prompt,
        direction: None,
        presentation_style: None,
        web_use_input: Some(false),
        prompt_label_header: "Prompt Label Header".to_string(),
        prompt_label_cancel: Some("Cancel".to_string()),
        prompt_label_photo: Some("Select Saved Image".to_string()),
        prompt_label_picture: Some("Open Camera".to_string()),
    })
    .await
}

async fn can_share() {
    let result = Share::can_share().await;
    let text = if result.value {
        "Sharing is possible"
    } else {
        "Sharing is not possible"
    };
    Toast::show(text).await
}

async fn do_share() {
    let options = ShareOptions {
        title: Some("title".to_string()),
        text: Some("text".to_string()),
        url: Some("https://github.com/wainwrightmark/capacitor_bindings".to_string()),
        dialog_title: Some("dialog title".to_string()),
        files: None,
    };
    let result = Share::share(options).await;

    Toast::show(format!("{result:?}")).await
}

async fn show_actions() -> () {
    let result = capacitor_bindings::action_sheet::ActionSheet::show_actions(&ShowActionsOptions {
        title: "Title".to_string(),
        message: Some("Message".to_string()),
        options: vec![
            ActionSheetButton {
                title: "Action 0 (Default)".to_string(),
                style: Some(ActionSheetButtonStyle::Default),
                icon: None,
            },
            ActionSheetButton {
                title: "Action 1 (Destructive)".to_string(),
                style: Some(ActionSheetButtonStyle::Destructive),
                icon: None,
            },
            ActionSheetButton {
                title: "Action 2 (Cancel)".to_string(),
                style: Some(ActionSheetButtonStyle::Cancel),
                icon: None,
            },
        ],
    })
    .await;

    Toast::show(toast::ShowOptions {
        text: format!("Chose action {}", result.index),
    })
    .await;
}

async fn schedule_notification(options: ScheduleOptions) -> () {
    log::info!("Scheduling local notification...");
    log::info!("{options:?}");
    let result = LocalNotifications::schedule(options).await;

    log::info!("Notification Scheduled {:?}", result.notifications);
}

async fn add_local_notification_received_listener() {
    LocalNotifications::add_received_listener(|schema| {
        spawn_local(Toast::show(format!("Notification Received: {}", schema.id)));
    })
    .await;
}

async fn add_action_performed_listener() {
    LocalNotifications::add_action_performed_listener(|action_performed| {
        spawn_local(Toast::show(format!(
            "Action Performed: {}",
            action_performed.action_id
        )));
    })
    .await;
}
