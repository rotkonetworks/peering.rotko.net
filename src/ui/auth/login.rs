use dioxus::prelude::*;

const LOGIN_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Login() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LOGIN_CSS}

        div {
            h1 { "This is login !" }
        }
    }
}
