use crate::ui::auth::login::Login;
use crate::ui::home::home::Home;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::document;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/resources/icon/favicon.ico");
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

#[component]
pub fn Navigation() -> Element {
    rsx! {
        div {
            id: "appbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Login {},
                "Login"
            }
        }

        Outlet::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    Login { },
}
