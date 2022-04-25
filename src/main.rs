pub mod api;
pub mod components;
pub mod hooks;

use components::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
