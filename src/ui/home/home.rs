use crate::ui::common::component::echo::Echo;
use crate::ui::common::component::hero::Hero;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
