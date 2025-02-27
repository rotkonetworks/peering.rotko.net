use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    #[props(optional)]
    class: Option<String>,

    children: Element,
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    let default_class = format!("grid {}", props.class.unwrap_or_default());

    rsx! {
        div {
            class: "{default_class}",
            {props.children}
        }
    }
}
