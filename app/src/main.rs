use dioxus::prelude::*;

use ui::app::app::App;
mod data;
mod domain;
mod ui;

fn main() {
    launch(App);
}