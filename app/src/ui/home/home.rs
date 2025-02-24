use crate::data::profile::Profile;
use crate::data::{create_credentials_repository, create_profile_repository};
use crate::ui::app::app::Route;
use dioxus::html::completions::CompleteWithBraces::code;
use dioxus::prelude::*;
use std::ops::Deref;
use ui::design::component::app_bar::TopAppBar;
use ui::design::component::button::{Button, IconButton};
use ui::design::component::icon::Icon;
use ui::design::component::image::Image;
use ui::design::component::text::Text;
use ui::design::reference;
use ui::foundation::column::Column;
use ui::foundation::HorizontalAlignment;

#[derive(Clone, PartialEq)]
enum HomeState {
    Loading,
    Success(Profile),
    Error,
}

/// Main screen of the app.
///
/// It shows information about Rotko Networks and the user's networks.
#[component]
pub fn HomeScreen() -> Element {
    let home_state = use_resource({
        move || async move {
            let credentials_repository = create_credentials_repository();

            if let Some(access_token) = credentials_repository.get_access_token() {
                let profile_repository = create_profile_repository(access_token);

                // // TODO Fake
                // return HomeState::Success(Profile {
                //     id: 1,
                //     name: "Michael Kayne".to_string(),
                //     given_name: "".to_string(),
                //     family_name: "".to_string(),
                //     email: "mock@example.com".to_string(),
                //     verified_user: false,
                //     verified_email: false,
                //     networks: vec![],
                // })

                match profile_repository.get().await {
                    Ok(profile) => return HomeState::Success(profile),
                    Err(_) => {}
                }
            }

            let navigator = navigator();
            navigator.replace("/login");
            HomeState::Loading
        }
    });

    rsx! {

        match home_state.read().deref() {
            Some(HomeState::Success(profile)) => {
                rsx! { Success { user_name: profile.name.clone() } }
            },
            _ => {
                let message = match home_state.read().deref() {
                    Some(HomeState::Error) => "Ops.. Something went wrong.".to_string(),
                    _ => "Loading...".to_string(),
                };
                rsx! { State { message } }
            }
        }

        // Echo {}
    }
}

/// Component to show the state loading and error
#[component]
fn State(message: String) -> Element {
    rsx! {
       div {
            class: "flex flex-col items-center justify-center h-screen bg-gray-200",

            Image {
               class: "mb-2 filter grayscale brightness-90 contrast-125",
               src: reference::image::LOGO,
               width: 64,
               height: 64
            }

            Text {
               class: "text-gray-400 italic",
               text: "Rotko Networks"
            }

            Text {
               class: "p-4 font-semibold",
               text: message
            }
        }
    }
}

#[component]
fn Success(user_name: String) -> Element {
    rsx! {
        div {
            class: "h-screen flex flex-col",

            TopAppBar {
                title: "Peering",
                actions: Some(
                    rsx! {
                        IconButton {
                            icon: Icon {
                                width: 24,
                                height: 24,
                                src: reference::icon::LOGOUT.to_string()
                            },
                            aria_label: "Logout",
                            on_click: move |_| {
                                let credentials_repository = create_credentials_repository();
                                credentials_repository.delete_access_token();
                                let navigator = navigator();
                                navigator.replace("/login");
                            }
                        }
                    }
                ),
            }

            div {
                class: "flex flex-col flex-1 items-center justify-center bg-gray-200",

                Image {
                   class: "mb-2 filter grayscale brightness-90 contrast-125",
                   src: reference::image::LOGO,
                   width: 64,
                   height: 64
                }

                Text {
                   class: "text-gray-400 italic",
                   text: "Rotko Networks"
                }

                Text {
                   class: "p-4 font-semibold",
                   text: format!("Welcome, {user_name}")
                }
            }
        }
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
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
