mod app;
pub mod network;
pub mod notifications;
pub mod listener;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("App started");
    yew::Renderer::<App>::new().render();
}
