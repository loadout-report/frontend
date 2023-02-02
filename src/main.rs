#![feature(result_option_inspect)]

mod app;
mod routes;
mod components;
mod client;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
