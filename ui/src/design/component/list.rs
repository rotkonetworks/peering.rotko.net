use crate::design::component::icon::Icon;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ListItemProps {
    /// Primary label for the list item.
    label: String,

    #[props(optional)]
    is_enabled: Option<bool>,

    /// Optional leading icon.
    #[props(optional)]
    icon: Option<Icon>,

    #[props(optional)]
    trailing_content: Option<Element>,

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
    let class = props
        .class
        .clone()
        .unwrap_or_else(|| "dxa-list-item".into());

    let is_enabled = props.is_enabled.unwrap_or(true); // Default to true

    let mut modifier = "flex items-center justify-between p-4".to_string();
    if is_enabled {
        modifier.push_str(" hover:bg-gray-200 active:bg-gray-300 cursor-pointer");
    } else {
        modifier.push_str(" opacity-50 cursor-not-allowed pointer-events-none");
    }

    rsx! {
        div {
            class: "{class} {modifier}",
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

            // Trailing content
            if let Some(trailing) = &props.trailing_content {
                {trailing}
            }
        }
    }
}
