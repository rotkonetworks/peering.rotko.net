use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    #[props(optional)]
    is_open: bool,

    #[props(optional, default = false)]
    align_right: bool,

    children: Element,
}

#[component]
pub fn Menu(props: MenuProps) -> Element {
    if !props.is_open {
        return rsx!();
    }

    let alignment_class = if props.align_right {
        "right-0" // Align to the right edge
    } else {
        "left-0" // Default alignment
    };

    rsx! {
        div {
            class: "absolute mt-2 w-48 bg-white shadow-lg rounded-lg border border-gray-200 z-50 {alignment_class}",
            {props.children}
        }
    }
}
