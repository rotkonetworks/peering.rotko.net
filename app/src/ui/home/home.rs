use crate::data::profile::{Network, Profile};
use crate::data::{create_credentials_repository, create_profile_repository};
use crate::ui::app::app::Route;
use dioxus::html::completions::CompleteWithBraces::code;
use dioxus::prelude::*;
use dioxus_charts::charts::pie::LabelPosition;
use dioxus_charts::{BarChart, LineChart, PieChart};
use std::ops::{Deref, Div};
use ui::design::component::app_bar::TopAppBar;
use ui::design::component::button::{Button, IconButton};
use ui::design::component::grid::Grid;
use ui::design::component::icon::Icon;
use ui::design::component::image::Image;
use ui::design::component::list::ListItem;
use ui::design::component::menu::Menu;
use ui::design::component::text::Text;
use ui::design::reference;
use ui::foundation::column::Column;
use ui::foundation::row::Row;
use ui::foundation::{Alignment, Arrangement};

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
                return HomeState::Success(Profile {
                    id: 1,
                    name: "Michael Kayne".to_string(),
                    given_name: "".to_string(),
                    family_name: "".to_string(),
                    email: "michael.kayne@example.com".to_string(),
                    verified_user: false,
                    verified_email: false,
                    networks: vec![
                        Network {
                            perms: 3,
                            asn: 65001,
                            name: "AT&T".to_string(),
                            id: 101,
                        },
                        Network {
                            perms: 5,
                            asn: 65002,
                            name: "Verizon Communications".to_string(),
                            id: 102,
                        },
                        Network {
                            perms: 2,
                            asn: 65003,
                            name: "T-Mobile USA".to_string(),
                            id: 103,
                        },
                    ],
                });

                // match profile_repository.get().await {
                //     Ok(profile) => return HomeState::Success(profile),
                //     Err(_) => {}
                // }
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
                                on_click: move |_| { menu_open.toggle() }
                            }

                            if menu_open(){
                                Menu {
                                    align_right: true,
                                    on_dismiss: move |_| {
                                        menu_open.set(false)
                                    },

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
                                                menu_open.set(false);
                                            }
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
            class: "flex flex-1 items-center justify-center bg-gray-200",

            Grid {
                class: "grid grid-cols-3 gap-4 max-w-4xl mx-auto",

                    Greeting { user_name: user_name }

                    Traffic {}

                    Bandwidth {}

                    Locations {}

                }
            }
        }
    }
}

#[component]
fn Greeting(user_name: String) -> Element {
    rsx! {
        Column {
            class: "bg-white w-full border border-gray-300 rounded-lg shadow-lg p-4",
            horizontal_alignment: Alignment::Center,
            vertical_arrangement: Arrangement::Center,

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

#[component]
fn Traffic() -> Element {
    let values: Vec<f32> = vec![10.0, 2.0];
    let total: f32 = values.iter().sum();

    let percentages: Vec<String> = values
        .iter()
        .map(|&v| format!("{:.1}%", (v / total) * 100.0))
        .collect();

    rsx! {
        Column {
            class: "bg-white w-full border border-gray-300 rounded-lg shadow-lg p-4",
            horizontal_alignment: Alignment::Start,
            vertical_arrangement: Arrangement::Start,

            Text {
               class: "font-semibold",
               text: "Traffic"
            }

            div {
                class: "relative flex items-center justify-center",

                PieChart {
                    width: "100%",
                    height: "100%",
                    start_angle: 50.0,
                    label_offset: 27.0,
                    label_position: LabelPosition::Outside,
                    donut: true,
                    donut_width: 30.0,
                    padding: 20.0,
                    series: values.clone(),
                    labels: vec!["".into(), "".into()],
                }

                Text {
                    class: "text-4xl text-gray-700 font-semibold absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2",
                    text: format!("{}k", total)
                }
            }

            Row {
                class: "flex w-full",
                vertical_alignment: Alignment::Center,

                div {
                    class: "w-3 h-3 bg-red-500 rounded-sm m-1",
                }

                Text {
                   class: "text-sm font-medium text-gray-700 py-2 mx-1 flex-grow",
                   text: "From Rotko"
                }

                 Text {
                   class: "text-sm font-medium text-gray-700 py-2 mx-1",
                   text: percentages.first().cloned().unwrap_or_else(|| "".into()),
                 }
            }

             Row {
                 class: "flex w-full",
                 vertical_alignment: Alignment::Center,

                 div {
                    class: "w-3 h-3 bg-red-800 rounded-sm m-1",
                 }

                 Text {
                   class: "text-sm font-medium text-gray-700 py-2 mx-1 flex-grow",
                   text: "To Rotko"
                 }

                 Text {
                   class: "text-sm font-medium text-gray-700 py-2 mx-1",
                   text: percentages.last().cloned().unwrap_or_else(|| "".into()),
                 }
            }
        }
    }
}

#[component]
fn Bandwidth() -> Element {
    rsx! {
        Column {
            class: "bg-white w-full border border-gray-300 rounded-lg shadow-lg p-4",
            horizontal_alignment: Alignment::Start,
            vertical_arrangement: Arrangement::Start,

            Text {
               class: "font-semibold",
               text: "Bandwidth"
            }
        }
    }
}

#[component]
fn Locations() -> Element {
    rsx! {
        Column {
            class: "col-span-3 bg-white w-full h-[300px] border border-gray-300 rounded-lg shadow-lg p-4",
            horizontal_alignment: Alignment::Start,
            vertical_arrangement: Arrangement::Start,

            Text {
               class: "font-semibold",
               text: "Locations"
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
