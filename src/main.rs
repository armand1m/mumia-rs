pub mod api;
pub mod components;
pub mod contexts;
pub mod hooks;

use components::root::Root;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Root>();
}
