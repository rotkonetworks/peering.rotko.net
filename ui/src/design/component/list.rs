use dioxus::prelude::*;
use crate::design::component::icon::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct ListItemProps {
    /// Primary label for the list item.
    label: String,

    /// Optional leading icon.
    #[props(optional)]
    icon: Option<Icon>,

    /// Optional trailing icon.
    #[props(optional)]
    trailing_icon: Option<Icon>,

    /// Optional CSS class for custom styling.
    #[props(optional)]
    class: Option<String>,

    /// Event handler for click events.
    #[props(optional)]
    on_click: EventHandler<MouseEvent>,

    /// Event handler for mouse enter events.
    #[props(optional)]
    on_mouse_enter: EventHandler<MouseEvent>,

    /// Event handler for mouse leave events.
    #[props(optional)]
    on_mouse_leave: EventHandler<MouseEvent>,

    /// Event handler for focus events.
    #[props(optional)]
    on_focus: EventHandler<FocusEvent>,
}

#[component]
pub fn ListItem(props: ListItemProps) -> Element {
    let class = props.class.clone().unwrap_or_else(|| "dxa-list-item".into());

    rsx! {
        div {
            class: "{class} flex items-center justify-between p-4 hover:bg-gray-200 active:bg-gray-300 cursor-pointer",
            onclick: move |e| props.on_click.call(e),
            onmouseenter: move |e| props.on_mouse_enter.call(e),
            onmouseleave: move |e| props.on_mouse_leave.call(e),
            onfocus: move |e| props.on_focus.call(e),

            // Leading content
            div {
                class: "flex items-center",
                if let Some(icon) = props.icon {
                    img {
                        src: "{icon.src}",
                        width: "{icon.width}",
                        height: "{icon.height}",
                        class: "mr-2 select-none",
                    }
                }

                span { class: "select-none", "{props.label}" }
            }

            // Trailing icon
            if let Some(trailing) = props.trailing_icon {
                img {
                    src: "{trailing.src}",
                    width: "{trailing.width}",
                    height: "{trailing.height}",
                    class: "ml-4 select-none",
                }
            }
        }
    }
}