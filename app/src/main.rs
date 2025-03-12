use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use ui::app::app::App;
mod data;
mod domain;
mod ui;

fn main() {
    dioxus_logger::init(Level::INFO).expect("Logger failed to initialize.");
    launch(App);
}