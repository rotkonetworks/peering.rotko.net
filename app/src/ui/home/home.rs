use crate::data::location::Location;
use crate::data::profile::{Network, Profile};
use crate::data::{
    create_credentials_repository, create_location_repository, create_profile_repository,
};
use crate::ui::app::app::Route::BandwidthScreen;
use dioxus::html::option::selected;
use dioxus::prelude::*;
use dioxus_charts::charts::pie::LabelPosition;
use dioxus_charts::{LineChart, PieChart};
use dioxus_material::IconKind::Segment;
use futures::try_join;
use std::ops::Deref;
use ui::design::component::app_bar::TopAppBar;
use ui::design::component::button::{Button, IconButton};
use ui::design::component::grid::Grid;
use ui::design::component::icon::Icon;
use ui::design::component::image::Image;
use ui::design::component::list::ListItem;
use ui::design::component::menu::Menu;
use ui::design::component::segmented_button::SegmentedButton;
use ui::design::component::segmented_button::SegmentedButtonType;
use ui::design::component::text::Text;
use ui::design::reference;
use ui::foundation::column::Column;
use ui::foundation::row::Row;
use ui::foundation::{Alignment, Arrangement};

#[derive(Clone, PartialEq)]
enum HomeState {
    Loading,
    Success(Profile, Vec<Location>, Vec<f32>),
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
                let profile_repository = create_profile_repository(access_token.clone());
                let location_repository = create_location_repository(access_token.clone());

                let traffic_data: Vec<f32> = vec![10.0, 2.0];

                return match try_join!(
                    profile_repository.get(access_token.clone()),
                    location_repository.get(access_token.clone())
                ) {
                    Ok((profile, locations)) => {
                        HomeState::Success(profile, locations, traffic_data)
                    }
                    Err(_) => HomeState::Error, // Handle failure case
                };
            }

            let navigator = navigator();
            navigator.replace("/login");
            HomeState::Loading
        }
    });

    rsx! {
        match home_state.read().deref() {
            Some(HomeState::Success(profile, locations, traffic_data)) => {
                rsx! {
                    Success {
                        profile: profile.clone(),
                        locations: locations.clone(),
                        traffic_data: traffic_data.clone()
                    }
                }
            },
            _ => {
                let message = match home_state.read().deref() {
                    Some(HomeState::Error) => "Ops.. Something went wrong.".to_string(),
                    _ => "Loading...".to_string(),
                };
                rsx! { State { message } }
            }
        }
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
fn Success(profile: Profile, locations: Vec<Location>, traffic_data: Vec<f32>) -> Element {
    let user_name = &profile.name;

    let mut selected_network = use_signal(|| profile.networks.first().cloned());

    let mut menu_open = use_signal(|| false);

    rsx! {
        div {
            class: "h-screen flex flex-col",

            TopAppBar {
                title: "Peering",
                actions: Some(
                    rsx! {
                        if let Some(current_network) = selected_network() {
                            div {
                                class: "relative inline-block",
                                Button {
                                    label: current_network.name,
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
                                credentials_repository.delete_refresh_token();
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

                    Traffic { data: traffic_data}

                    Bandwidth {}

                    Locations { data: locations }
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
fn Traffic(data: Vec<f32>) -> Element {
    let total: f32 = data.iter().sum();

    let percentages: Vec<String> = data
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
                    series: data.clone(),
                    labels: vec!["".into(), "".into()],
                }

                Text {
                    class: "text-2xl sm:text-4xl text-gray-700 font-semibold absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2",
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
    let mut is_bits = use_signal(|| true);

    let series = vec![
        vec![
            1.12, 1.05, 1.08, 1.02, 0.95, 0.98, 0.90, 0.85, 0.50, 0.82, 1.00, 1.10, 1.15, 1.12,
            1.08, 1.05, 1.00, 0.98, 0.92, 0.85, 0.78, 0.72, 0.68, 0.60,
        ],
        vec![
            1.00, 0.98, 0.95, 0.92, 0.90, 0.85, 0.80, 0.75, 0.78, 0.60, 0.75, 0.88, 0.92, 1.00,
            1.05, 1.08, 1.10, 1.12, 1.08, 1.02, 0.95, 0.90, 0.85, 0.80,
        ],
    ];

    let labels: Vec<String> = (0..24)
        .rev()
        .map(|h| {
            if h % 4 == 0 {
                format!("{:02} {}", h % 12 + 1, if h >= 12 { "PM" } else { "AM" })
            } else {
                "".into()
            }
        })
        .collect();

    rsx! {
        Column {
            class: "flex bg-white w-full border border-gray-300 rounded-lg shadow-lg p-2",
            horizontal_alignment: Alignment::Start,
            vertical_arrangement: Arrangement::Start,

            Row {
                class: "flex w-full",

                Text {
                    class: "font-semibold p-2 flex-grow cursor-pointer",
                    text: "Bandwidth",
                    on_click: move |_| {
                        let navigator = navigator();
                        navigator.push("/bandwidth");
                    }
                }

                Row {
                    SegmentedButton {
                        selected: is_bits(),
                        label: "Bits",
                        segment_type: SegmentedButtonType::Start,
                        on_click: move |_| is_bits.set(true),
                    }

                    SegmentedButton {
                        selected: !is_bits(),
                        label: "Packets",
                        segment_type: SegmentedButtonType::End,
                        on_click: move |_| is_bits.set(false),
                    }
                }
            }

            div {
                class: "relative flex items-center justify-center py-4",

                LineChart {
                    width: "100%",
                    height: "100%",
                    viewbox_width: 600,
                    viewbox_height: 290,
                    padding_top: 30,
                    padding_left: 50,
                    padding_right: 90,
                    padding_bottom: 30,
                    show_line_labels: false,
                    show_grid: false,
                    show_dotted_grid: false,
                    show_grid_ticks: false,
                    show_dots: false,
                    label_interpolation: (|v| format!("{:.1}", v)) as fn(f32) -> String,
                    series: series,
                    labels: labels,
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
            }
        }
    }
}

#[component]
fn Locations(data: Vec<Location>) -> Element {
    rsx! {
        Column {
            class: "col-span-3 bg-white w-full min-h-[260px] border border-gray-300 rounded-lg shadow-lg p-4",
            horizontal_alignment: Alignment::Start,
            vertical_arrangement: Arrangement::Start,

            Text {
               class: "font-semibold pb-4",
               text: "Locations"
            }

            div {
                class: "overflow-x-auto",
                table {
                    class: "min-w-full border-collapse border border-gray-300",

                    thead {
                        class: "bg-gray-200",
                        tr {
                            th { class: "text-left min-w-[200px] border border-gray-300 px-4 py-2", "Name" }
                            th { class: "border border-gray-300 px-4 py-2", "Rotko IP Address" }
                            th { class: "border border-gray-300 px-4 py-2", "Peer IP Address" }
                            th { class: "w-[140px] border border-gray-300 px-4 py-2", "Prefix Sent to Peer" }
                            th { class: "w-[140px] border border-gray-300 px-4 py-2", "Prefix Received from Peer" }
                            th { class: "w-max-[140px] border border-gray-300 px-4 py-2", "Session Established" }
                        }
                    }

                    tbody {
                        for peer in data {
                            tr {
                                td { class: "border border-gray-300 px-4 py-2", "{peer.name}" }
                                td { class: "text-center border border-gray-300 px-4 py-2", "{peer.rotko_ip}" }
                                td { class: "text-center border border-gray-300 px-4 py-2", "{peer.peer_ip}" }
                                td { class: "text-center border border-gray-300 px-4 py-2", "{peer.prefix_sent}" }
                                td { class: "text-center border border-gray-300 px-4 py-2", "{peer.prefix_received}" }
                                td { class: "text-center border border-gray-300 px-4 py-2", "{peer.session_established}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
