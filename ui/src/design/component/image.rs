use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ImageProps {
    #[props(optional, default = "dxa-image".into())]
    class: String,

    src: String,

    width: u32,

    height: u32,
}

#[component]
pub fn Image(props: ImageProps) -> Element {
    rsx! {
        img {
            class: props.class,
            src: props.src,
            width: "{props.width}",
            height: "{props.height}",
        }
    }
}