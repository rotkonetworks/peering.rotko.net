use dioxus::prelude::*;
use ui::design::component::button::Button;
use ui::foundation::column::Column;
use ui::foundation::{HorizontalAlignment, VerticalArrangement};
use ui::foundation::row::Row;

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-screen",

            Column {
                class: "bg-gray-200 w-[300px] h-[300px]",
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_arrangement: VerticalArrangement::Center,

                h1 { "Peering Portal!" }

                h2 { "Sign In With:" }

                Button {
                    label: "Click Me!",
                    on_click: move |_| {
                        println!("Button clicked.")
                    }
                }
            }
        }
    }
}