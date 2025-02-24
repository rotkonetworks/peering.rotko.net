use dioxus::prelude::*;
use crate::design::component::text::Text;
use crate::foundation::row::Row;

#[derive(Props, Clone, PartialEq)]
pub struct TopAppBarProps {
    #[props(optional, default = "dxa-top-app-bar".into())]
    class: String,

    title: String,

    #[props(optional)]
    navigation: Option<Element>,

    #[props(optional)]
    actions: Option<Element>,
}

#[component]
pub fn TopAppBar(props: TopAppBarProps) -> Element {
    rsx! {
        Row {
            class: "{props.class} bg-white shadow-md",

            if let Some(nav_icon) = &props.navigation {
                div { class: "nav-icon p-2", {nav_icon} }
            }

            div {
                class: "flex-1 flex flex-row items-center h-[56px] px-4",

                Text {
                    class: "text-lg font-medium text-gray-900",
                    text: &props.title
                }
            }

            if let Some(actions) = &props.actions {
                Row { class: "actions p-2", {actions} }
            }
        }
    }
}
