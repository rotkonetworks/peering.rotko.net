use crate::data::{create_auth_repository, create_credentials_repository};
use crate::domain::oauth::{build_oauth_url, build_oauth_url_pcke, generate_code_verifier, generate_random_state, get_redirect_uri, CLIENT_ID};
use crate::ui::app::app::Route;
use dioxus::prelude::*;
use std::ops::Deref;
use std::string::ToString;
use ui::design::component::app_bar::TopAppBar;
use ui::design::component::grid::Grid;
use ui::design::component::icon::Icon;
use ui::design::component::image::Image;
use ui::design::component::list::ListItem;
use ui::design::component::text::Text;
use ui::design::reference;
use ui::foundation::column::Column;
use ui::foundation::{Alignment, Arrangement};

#[derive(Clone, PartialEq)]
enum BandwidthState {
    None,
    Success,
    Error,
}

#[component]
pub fn BandwidthScreen() -> Element {
    let navigator = navigator();

    rsx! {

        div {
            class: "flex flex-1 items-center justify-center bg-gray-200",

            Grid {
                class: "grid grid-cols-3 gap-4 max-w-4xl mx-auto",
            }
        }
    }
}
