use dioxus::prelude::*;
use crate::ui::app::app::Route;

const APPBAR_CSS: Asset = asset!("/assets/styling/appbar.css");

#[component]
pub fn AppBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: APPBAR_CSS }

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
