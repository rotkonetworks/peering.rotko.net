use crate::design::component::button::ButtonProps;
use crate::design::component::icon::Icon;
use crate::foundation::{HorizontalAlignment, VerticalArrangement};
use dioxus::core_macro::{component, rsx, Props};
use dioxus::dioxus_core::Element;
use dioxus::events::{FocusEvent, MouseEvent};
use dioxus::hooks::use_signal;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ColumnProps {
    #[props(optional)]
    class: Option<String>,

    #[props(optional, default = VerticalArrangement::Start)]
    vertical_arrangement: VerticalArrangement,

    #[props(optional, default = HorizontalAlignment::Start)]
    horizontal_alignment: HorizontalAlignment,

    children: Element,
}

#[component]
pub fn Column(props: ColumnProps) -> Element {
    let vertical = match props.vertical_arrangement {
        VerticalArrangement::Center => "justify-center",
        VerticalArrangement::Between => "justify-between",
        VerticalArrangement::Around => "justify-around",
        VerticalArrangement::Evenly => "justify-evenly",
        VerticalArrangement::Start => "justify-start",
    };

    let horizontal = match props.horizontal_alignment {
        HorizontalAlignment::Center => "items-center",
        HorizontalAlignment::End => "items-end",
        HorizontalAlignment::Start => "items-start",
    };

    let default_class = format!(
        "flex flex-col {vertical} {horizontal} {}",
        props.class.unwrap_or_default()
    );

    rsx! {
        div {
            class: "{default_class}",
            {props.children}
        }
    }
}
