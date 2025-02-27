use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {

    #[props(optional)]
    on_dismiss: EventHandler<FocusEvent>,

    #[props(optional, default = false)]
    align_right: bool,

    children: Element,
}

#[component]
pub fn Menu(props: MenuProps) -> Element {

    let alignment_class = if props.align_right {
        "right-0" // Align to the right edge
    } else {
        "left-0" // Default alignment
    };

    rsx! {
        div {
            class: "absolute mt-2 w-48 bg-white shadow-lg rounded-lg border border-gray-200 z-50 {alignment_class}",
            tabindex: 0,
            onmounted: move |event| async move {
                event.set_focus(true);
            },
            onfocusout: props.on_dismiss,
            {props.children}
        }
    }
}
