use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::document;
use dioxus::prelude::*;
use crate::ui::common::component::app_bar;
use crate::ui::home::home::Home;
use crate::ui::auth::login::Login;
use crate::ui::app::app::app_bar::AppBar;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppBar)]
    #[route("/")]
    Home {},
    #[route("/login")]
    Login { },
}