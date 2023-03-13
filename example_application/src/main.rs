mod app;
#[macro_use]
pub mod listener;
pub mod network;
pub mod notifications;
pub mod screen_reader;
pub mod preferences;

pub mod app_funcs;
pub mod haptics;
use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("App started");
    yew::Renderer::<App>::new().render();
}
