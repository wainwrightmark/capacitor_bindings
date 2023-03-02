mod app;
mod safe_area;
use app::App;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("App started");
    yew::Renderer::<App>::new().render();
}
