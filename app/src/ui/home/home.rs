use crate::data::profile::{Network, Profile};
use crate::data::{create_credentials_repository, create_profile_repository};
use crate::ui::app::app::Route;
use dioxus::html::completions::CompleteWithBraces::code;
use dioxus::prelude::*;
use std::ops::Deref;
use ui::design::component::app_bar::TopAppBar;
use ui::design::component::button::{Button, IconButton};
use ui::design::component::icon::Icon;
use ui::design::component::image::Image;
use ui::design::component::list::ListItem;
use ui::design::component::menu::Menu;
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

                // TODO Fake
                // return HomeState::Success(Profile {
                //     id: 1,
                //     name: "Michael Kayne".to_string(),
                //     given_name: "".to_string(),
                //     family_name: "".to_string(),
                //     email: "michael.kayne@example.com".to_string(),
                //     verified_user: false,
                //     verified_email: false,
                //     networks: vec![
                //         Network {
                //             perms: 3,
                //             asn: 65001,
                //             name: "AT&T".to_string(),
                //             id: 101,
                //         },
                //         Network {
                //             perms: 5,
                //             asn: 65002,
                //             name: "Verizon Communications".to_string(),
                //             id: 102,
                //         },
                //         Network {
                //             perms: 2,
                //             asn: 65003,
                //             name: "T-Mobile USA".to_string(),
                //             id: 103,
                //         },
                //     ],
                // });

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
                rsx! { Success { profile: profile.clone() } }
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
fn Success(profile: Profile) -> Element {
    let user_name = &profile.name;

    let mut selected_network = use_signal(|| profile.networks.first().cloned());

    let mut menu_open = use_signal(|| false);

    let toggle_menu = move |_| {
        menu_open.set(!menu_open());
    };

    let network_names: Vec<String> = profile
        .networks
        .iter()
        .map(|network| network.name.clone())
        .collect();

    rsx! {
        div {
            class: "h-screen flex flex-col",

            TopAppBar {
                title: "Peering",
                actions: Some(
                    rsx! {
                         div {
                            class: "relative inline-block",
                            Button {
                                label: selected_network().unwrap().name,
                                trailing_icon: Icon {
                                    width: 24,
                                    height: 24,
                                    src: reference::icon::CHEVRON_DOWN.to_string()
                                },
                                on_click: toggle_menu
                            }
                            Menu {
                                is_open: menu_open(),
                                align_right: true,

                                for network in profile.networks.clone() {
                                    ListItem {
                                        label: &network.name,
                                        trailing_content: Some(
                                            rsx! {
                                                input {
                                                    class: "cursor-pointer",
                                                    r#type: "radio",
                                                    checked: network.asn == selected_network().unwrap().asn
                                                }
                                            }
                                        ),
                                        on_click: move |_| {
                                            selected_network.set(Some(network.clone()));
                                        }
                                    }
                                }
                            }
                        }

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
