mod app;
#[macro_use]
pub mod listener;
pub mod browser;
pub mod clipboard;
pub mod network;
pub mod notifications;
pub mod preferences;
pub mod rate;
pub mod screen_reader;
pub mod splash;
pub mod toast;

pub mod app_funcs;
pub mod haptics;
use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("App started");

    #[cfg(any(feature = "ios", feature = "android"))]
    app::do_async(|| capacitor_bindings::splash_screen::SplashScreen::hide(1000.0));

    yew::Renderer::<App>::new().render();
}
