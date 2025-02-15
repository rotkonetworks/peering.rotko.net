use crate::ui::app::app::Route;
use dioxus::prelude::*;
use ui::design::component::button::Button;
use ui::design::component::icon::Icon;
use ui::design::component::image::Image;
use ui::design::component::list::ListItem;
use ui::design::component::text::Text;
use ui::design::reference;
use ui::foundation::column::Column;
use ui::foundation::row::Row;
use ui::foundation::{HorizontalAlignment, VerticalArrangement};

#[component]
pub fn Login() -> Element {
    let nav = navigator();

    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen bg-gray-200",

            Image {
                class: "mb-8 filter grayscale brightness-90 contrast-125",
                src: reference::image::LOGO,
                width: 64,
                height: 64
            }

            Column {
                class: "bg-white w-[480px] border border-gray-300 rounded-lg shadow-lg p-8",
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_arrangement: VerticalArrangement::Center,

                Text {
                    class: "text-gray-400 italic",
                    text: "Rotko Networks"
                }

                Text {
                    class: "text-4xl p-4",
                    text: "Peering Portal"
                }

                Text {
                    class: "p-2 text-center",
                    text: "Explore real-time peering information for the Rotko Network. Stay informed on connection status and network performance."
                }

                Text {
                    class: "p-4 font-semibold",
                    text: "Continue with:"
                }

                ListItem {
                    class: "w-[300px] rounded-lg border border-gray-300 bg-gray-100",
                    icon: Icon {
                        width: 24,
                        height: 24,
                        src: reference::icon::OIDC.to_string()
                    },
                    label: "PeeringDB",
                    trailing_icon: Icon {
                        width: 24,
                        height: 24,
                        src: reference::icon::CHEVRON_RIGHT.to_string()
                    },
                    on_click: move |_| {
                        // TODO replace for PeeringDB login
                        nav.push(Route::Home {});
                    }
                }
            }
        }
    }
}
