use crate::design::component::button::ButtonProps;
use crate::design::component::icon::Icon;
use crate::foundation::{Alignment, Arrangement};
use dioxus::core_macro::{component, rsx, Props};
use dioxus::dioxus_core::Element;
use dioxus::events::{FocusEvent, MouseEvent};
use dioxus::hooks::use_signal;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ColumnProps {
    #[props(optional)]
    class: Option<String>,

    #[props(optional, default = Arrangement::Start)]
    vertical_arrangement: Arrangement,

    #[props(optional, default = Alignment::Start)]
    horizontal_alignment: Alignment,

    children: Element,
}

#[component]
pub fn Column(props: ColumnProps) -> Element {
    let vertical = match props.vertical_arrangement {
        Arrangement::Center => "justify-center",
        Arrangement::Between => "justify-between",
        Arrangement::Around => "justify-around",
        Arrangement::Evenly => "justify-evenly",
        Arrangement::Start => "justify-start",
    };

    let horizontal = match props.horizontal_alignment {
        Alignment::Center => "items-center",
        Alignment::End => "items-end",
        Alignment::Start => "items-start",
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
