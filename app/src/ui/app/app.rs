use crate::ui::auth::login::LoginScreen;
use crate::ui::home::home::HomeScreen;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::document;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/resources/icon/favicon.svg");
const TAILWIND_CSS: Asset = asset!("/resources/style/tailwind.css");
const MAIN_CSS: Asset = asset!("/resources/style/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    HomeScreen {},

    #[route("/login?:code&:state")]
    LoginScreen { code: Option<String>, state: Option<String> },
}
