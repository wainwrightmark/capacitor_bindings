use std::fmt::Debug;
use std::future::Future;

use capacitor_bindings::action_sheet::*;
use capacitor_bindings::helpers::Error;
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

use crate::network::NetworkView;
use crate::notifications::NotificationView;
use crate::app_funcs::AppView;

#[function_component(App)]
pub fn app() -> Html {
    let status_block: Html;
    #[cfg(any(feature = "android",))]
    {
        status_block = html! {
            <details>
            <summary>
                {"Status Bar"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_| do_and_toast_result(||{StatusBar::set_style(Style::Light)})}> {"Set Status Light"}</button>
                <button onclick={|_| do_and_toast_result(||{StatusBar::set_style(Style::Dark)})}> {"Set Status Dark"}</button>
                <button onclick={|_| do_and_toast_result(||{StatusBar::set_background_color("#22DD44")})}> {"Set Status Background Green"}</button>
                <button onclick={|_| do_and_toast_result(StatusBar::hide)}> {"Hide Status"}</button>
                <button onclick={|_| do_and_toast_result(StatusBar::show)}> {"Show Status"}</button>
            </div>
            </details>
        };
    };
    #[cfg(all(any(feature = "ios"), not(any(feature = "android"))))]
    {
        status_block = html! {
            <details>
            <summary>
                {"Status Bar"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <button onclick={|_| do_and_toast_result(||{StatusBar::set_style(Style::Light)})}> {"Set Status Light"}</button>
                <button onclick={|_| do_and_toast_result(||{StatusBar::set_style(Style::Dark)})}> {"Set Status Dark"}</button>
                <button onclick={|_| do_and_toast_result(StatusBar::hide)}> {"Hide Status"}</button>
                <button onclick={|_| do_and_toast_result(StatusBar::show)}> {"Show Status"}</button>
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
            <button onclick={|_| spawn_local(async{Toast::show("Hello Toast").await.expect("Should be able to show toast")})}> {"Show Toast"}</button>

            <button onclick={|_| show_actions()}> {"Show Actions"}</button>

            <NotificationView/>
            <AppView/>
            <details>
                <summary>
                    {"Share"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_|do_and_toast_result(Share::can_share)}> {"Can Share?"}</button>
                    <button onclick={|_|do_share()}> {"Share"}</button>
                </div>
            </details>

            <NetworkView/>

            <details>
                <summary>
                    {"Camera"}
                    <span class="icon">{"↓"}</span>
                </summary>
                <div style="display: flex; flex-direction: column;">
                    <button onclick={|_| get_photo()}> {"Get Photo"}</button>
                    <button onclick={|_|do_and_toast_result(Camera::check_permissions)}> {"Check Permissions"}</button>
                    <button onclick={|_|request_permissions()}> {"Request Permissions"}</button>
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


            </div>
        </main>
    }
}

async fn show_toast_or_panic_async(options: impl Into<toast::ShowOptions>) {
    Toast::show(options)
        .await
        .expect("Should Be able to show toast")
}

pub fn show_toast_or_panic(options: impl Into<toast::ShowOptions> + 'static) {
    spawn_local(show_toast_or_panic_async(options))
}

pub fn do_and_toast_result<
    T: Debug + 'static,
    Fut: Future<Output = Result<T, Error>>,
    F: Fn() -> Fut + 'static,
>(
    f: F,
) {
    spawn_local(async move {
        let r = f().await;

        match r {
            Ok(result) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<()>() {
                    log::info!("Action successful");
                } else {
                    log::info!("{result:?}");
                    show_toast_or_panic_async(format!("{result:?}")).await
                }
            }
            Err(err) => {
                log::error!("{err:?}");
                show_toast_or_panic_async(format!("{err}")).await
            }
        }
    })
}


pub fn do_async<
    Fut: Future<Output = Result<(), Error>>,
    F: Fn() -> Fut + 'static,
>(
    f: F,
) {
    spawn_local(async move {
        let r = f().await;

        match r {
            Ok(_) => {

            }
            Err(err) => {
                log::error!("{err:?}");
                show_toast_or_panic_async(format!("{err}")).await
            }
        }
    })
}





fn request_permissions() {
    #[cfg(any(feature = "android", feature = "ios"))]
    {
        do_and_toast_result(|| {
            Camera::request_permissions(CameraPluginPermissions {
                permissions: vec![CameraPermissionType::Camera, CameraPermissionType::Photos],
            })
        })
    }
    #[cfg(not(any(feature = "android", feature = "ios")))]
    {
        show_toast_or_panic("Cannot request permissions except on android or ios")
    }
}

fn get_photo() {
    do_and_toast_result(|| {
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
    })
}

fn do_share() {
    do_and_toast_result(|| {
        let options = ShareOptions {
            title: Some("title".to_string()),
            text: Some("text".to_string()),
            url: Some("https://github.com/wainwrightmark/capacitor_bindings".to_string()),
            dialog_title: Some("dialog title".to_string()),
            files: None,
        };
        Share::share(options)
    })
}

fn show_actions() {
    do_and_toast_result(|| {
        capacitor_bindings::action_sheet::ActionSheet::show_actions(ShowActionsOptions {
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
    });
}

fn add_local_notification_received_listener() {
    do_and_toast_result(|| {
        LocalNotifications::add_received_listener(|schema| {
            show_toast_or_panic(format!("Notification Received: {}", schema.id))
        })
    });
}

fn add_action_performed_listener() {
    do_and_toast_result(|| {
        LocalNotifications::add_action_performed_listener(|action_performed| {
            show_toast_or_panic(format!("Action Performed: {}", action_performed.action_id));
        })
    });
}

