use dioxus::prelude::*;
use crate::design::component::button::Button;
use crate::design::component::icon::Icon;
use crate::design::component::text::Text;

#[derive(PartialEq, Clone, Copy)]
pub enum SegmentedButtonType {
    Start,
    Middle,
    End,
}

#[derive(Props, Clone, PartialEq)]
pub struct SegmentedButtonProps {
    #[props(optional, default = "dxa-button".into())]
    class: String,

    label: String,

    #[props(optional, default = false)]
    selected: bool,

    #[props(optional, default = SegmentedButtonType::Middle)]
    segment_type: SegmentedButtonType,

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

#[component]
pub fn SegmentedButton(props: SegmentedButtonProps) -> Element {

    let shape_class = match props.segment_type {
        SegmentedButtonType::Start => "rounded-l-lg rounded-r-none",
        SegmentedButtonType::End => "rounded-r-lg rounded-l-none",
        _ => "rounded-none",
    };

    let selected_class = if props.selected {
        "bg-blue-500 hover:bg-blue-500 active:bg-blue-500 text-white"
    } else {
        "bg-gray-200 hover:bg-gray-200 active:bg-gray-300"
    };

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
            class: "{selected_class} {shape_class} px-3 py-2 text-gray-600 rounded-lg flex items-center gap-1 cursor-pointer font-semibold {props.class}",
            onclick: on_click,
            onmouseenter: move |data| props.on_mouse_enter.call(data),
            onmouseleave: move |data| props.on_mouse_leave.call(data),
            onfocus: move |data| props.on_focus.call(data),
            aria_pressed: aria_pressed_val,
            aria_label: aria_label_val,
            class: "flex-1 {shape_class} {selected_class}",

            Text {
                class: "align-middle",
                text: props.label
            }
        }
    }
}
