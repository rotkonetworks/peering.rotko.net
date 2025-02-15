use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TextProps {
    /// The text content to display.
    text: String,

    /// Optional CSS class for styling.
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

    /// Optional ARIA label for accessibility.
    #[props(optional)]
    aria_label: Option<String>,
}

#[component]
pub fn Text(props: TextProps) -> Element {
    let class = props.class.clone().unwrap_or_else(|| "dxa-text".into());

    rsx! {
        span {
            class: "{class}",
            aria_label: props.aria_label.clone(),
            onclick: move |e| props.on_click.call(e),
            onmouseenter: move |e| props.on_mouse_enter.call(e),
            onmouseleave: move |e| props.on_mouse_leave.call(e),
            "{props.text}"
        }
    }
}