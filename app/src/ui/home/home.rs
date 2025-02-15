use dioxus::prelude::*;
use ui::design::reference;

#[component]
pub fn Home() -> Element {
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