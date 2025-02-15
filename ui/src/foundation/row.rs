use crate::foundation::{HorizontalAlignment, VerticalArrangement};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RowProps {
    #[props(optional)]
    class: Option<String>,

    #[props(optional, default = VerticalArrangement::Start)]
    vertical_arrangement: VerticalArrangement,

    #[props(optional, default = HorizontalAlignment::Start)]
    horizontal_alignment: HorizontalAlignment,

    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
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
