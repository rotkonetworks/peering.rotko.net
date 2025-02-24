use dioxus::prelude::*;
use crate::design::component::icon::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(optional, default = "dxa-button".into())]
    class: String,
    label: String,
    icon: Option<Icon>,

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
///
///
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

    let on_mouse_enter = move |data| props.on_mouse_enter.call(data);
    let on_mouse_leave = move |data| props.on_mouse_leave.call(data);
    let on_focus = move |data| props.on_focus.call(data);

    let aria_pressed_val = match props.on_toggled.is_some() {
        true => match is_toggled() {
            true => Some("true"),
            false => Some("false"),
        },
        false => None,
    };

    let toggled_val = match props.on_toggled.is_some() {
        true => match is_toggled() {
            true => Some("true"),
            false => Some("false"),
        },
        false => None,
    };

    let aria_label_val = match props.icon.is_some() {
        true => Some(props.label.clone()),
        false => None,
    };

    rsx! {
        button {
            class: "{props.class} px-4 py-2 rounded-lg hover:bg-gray-200",
            // Events
            onclick: on_click,
            onmouseenter: on_mouse_enter,
            onmouseleave: on_mouse_leave,
            onfocus: on_focus,
            // Aria
            aria_pressed: aria_pressed_val,
            aria_label: aria_label_val,

            "toggled": toggled_val,
            if let Some(icon) = props.icon {
                img {
                    src: icon.src,
                    width: "{icon.width}",
                    height: "{icon.height}",
                 }
             } else {
                "{props.label}"
            },
        }
    }
}

use dioxus::prelude::*;

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

    let aria_label = props.aria_label.clone().unwrap_or_else(|| "icon button".into());

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

            img {
                src: props.icon.src,
                width: "{props.icon.width}",
                height: "{props.icon.height}",
            }
        }
    }
}
