use crate::data::create_credentials_repository;
use crate::ui::app::app::Route;
use dioxus::html::completions::CompleteWithBraces::code;
use dioxus::prelude::*;
use ui::design::reference;

#[derive(Clone, PartialEq)]
enum HomeState {
    None,
    Authenticated,
    Error,
}

#[component]
pub fn HomeScreen() -> Element {
    let home_state = use_resource({
        move || async move {
            let credentials_repository = create_credentials_repository();

            let access_token = credentials_repository.get_access_token();
            if access_token.is_none() {
                let navigator = navigator();
                navigator.replace("/login");
                HomeState::None
            } else {
                HomeState::Authenticated
            }
        }
    });

    rsx! {
        Hero {}
        Echo {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: reference::image::HEADER, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
            }
        }
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo anything...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
