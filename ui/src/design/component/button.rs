use crate::design::component::icon::Icon;
use crate::design::component::image::Image;
use crate::design::component::text::Text;
use dioxus::prelude::*;

/// Button properties.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(optional, default = "dxa-button".into())]
    class: String,

    label: String,

    #[props(optional, default = "gray-700".into())]
    color: String,

    #[props(optional)]
    leading_icon: Option<Icon>,

    #[props(optional)]
    trailing_icon: Option<Icon>,

    #[props(optional)]
    on_click: EventHandler<MouseEvent>,

    #[props(optional)]
    on_mouse_enter: EventHandler<MouseEvent>,

    #[props(optional)]
    on_mouse_leave: EventHandler<MouseEvent>,

    #[props(optional)]
    on_focus: EventHandler<FocusEvent>,

    /// Event that is fired whenever the toggled state of the button changes.
    ///
    /// Supplying this event handler will convert this button to a toggle button.
    on_toggled: Option<EventHandler<bool>>,
}

/// The `Button` ARIA pattern.
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let mut is_toggled = use_signal(|| false);

    let on_click = move |data| {
        if let Some(toggled_e) = props.on_toggled {
            is_toggled.toggle();
            let value = is_toggled();
            toggled_e.call(value);
        }
        props.on_click.call(data);
    };

    let aria_pressed_val = props.on_toggled.is_some().then(|| is_toggled().to_string());
    let aria_label_val = Some(props.label.clone());

    rsx! {
        button {
        class: "{props.class} px-3 py-2 rounded-lg flex items-center gap-1 hover:bg-gray-200 active:bg-gray-300 cursor-pointer font-semibold",
            // Events
            onclick: on_click,
            onmouseenter: move |data| props.on_mouse_enter.call(data),
            onmouseleave: move |data| props.on_mouse_leave.call(data),
            onfocus: move |data| props.on_focus.call(data),
            // Aria
            aria_pressed: aria_pressed_val,
            aria_label: aria_label_val,

            if let Some(leading_icon) = props.leading_icon {
                Image {
                    class: "fill-gray-600",
                    src: leading_icon.src,
                    width: leading_icon.width,
                    height: leading_icon.height
                }
            }

            Text {
                class: "align-middle text-gray-600",
                text: props.label
            }

            if let Some(trailing_icon) = props.trailing_icon {
                Image {
                    class: "fill-gray-600",
                    src: trailing_icon.src,
                    width: trailing_icon.width,
                    height: trailing_icon.height
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct IconButtonProps {
    #[props(optional, default = "dxa-icon-button".into())]
    class: String,

    icon: Icon,

    #[props(optional)]
    on_click: EventHandler<MouseEvent>,

    #[props(optional)]
    on_mouse_enter: EventHandler<MouseEvent>,

    #[props(optional)]
    on_mouse_leave: EventHandler<MouseEvent>,

    #[props(optional)]
    on_focus: EventHandler<FocusEvent>,

    #[props(optional)]
    aria_label: Option<String>,
}

#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
    let aria_label = props
        .aria_label
        .clone()
        .unwrap_or_else(|| "icon button".into());

    rsx! {
        button {
            class: "{props.class} w-10 h-10 flex items-center justify-center rounded-full hover:bg-gray-200 active:bg-gray-300 cursor-pointer",
            // Events
            onclick: move |e| props.on_click.call(e),
            onmouseenter: move |e| props.on_mouse_enter.call(e),
            onmouseleave: move |e| props.on_mouse_leave.call(e),
            onfocus: move |e| props.on_focus.call(e),
            // Aria
            aria_label: "{aria_label}",
            title: "{aria_label}", // Display the label on hover as a tooltip

            Image {
                class: "fill-gray-600",
                src: props.icon.src,
                width: props.icon.width,
                height: props.icon.height,
            }
        }
    }
}
