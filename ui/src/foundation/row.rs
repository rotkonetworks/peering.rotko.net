use crate::foundation::{Alignment, Arrangement};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RowProps {
    #[props(optional)]
    class: Option<String>,

    #[props(optional, default = Alignment::Start)]
    vertical_alignment: Alignment,

    #[props(optional, default = Arrangement::Start)]
    horizontal_arrangement: Arrangement,

    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    let vertical = match props.vertical_alignment {
        Alignment::Center => "items-center",
        Alignment::End => "items-end",
        Alignment::Start => "items-start",
    };

    let horizontal = match props.horizontal_arrangement {
        Arrangement::Center => "justify-center",
        Arrangement::Between => "justify-between",
        Arrangement::Around => "justify-around",
        Arrangement::Evenly => "justify-evenly",
        Arrangement::Start => "justify-start",
    };

    let default_class = format!(
        "flex flex-row {vertical} {horizontal} {}",
        props.class.unwrap_or_default()
    );

    rsx! {
        div {
            class: "{default_class}",
            {props.children}
        }
    }
}
