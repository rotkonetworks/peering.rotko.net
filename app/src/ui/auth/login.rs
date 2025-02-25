use crate::data::{create_auth_repository, create_credentials_repository};
use crate::domain::oauth::{
    build_oauth_url, generate_code_challenge, generate_code_verifier, generate_random_state,
    get_redirect_uri, CLIENT_ID,
};
use crate::ui::app::app::Route;
use dioxus::prelude::server_fn::client;
use dioxus::prelude::*;
use reqwest::Client;
use std::fs::read_dir;
use std::ops::{Deref, Not};
use std::string::ToString;
use ui::design::component::icon::Icon;
use ui::design::component::image::{Image, ImageProps};
use ui::design::component::list::ListItem;
use ui::design::component::text::Text;
use ui::design::reference;
use ui::foundation::column::Column;
use ui::foundation::row::Row;
use ui::foundation::{HorizontalAlignment, VerticalArrangement};

#[derive(Clone, PartialEq)]
enum LoginState {
    None,
    Success,
    Error,
}

#[component]
pub fn LoginScreen(code: String, state: String) -> Element {
    let navigator = navigator();

    let state_clone = state.clone();

    let login_state = use_resource({
        move || {
            let oauth_code = code.clone();
            let oauth_state = state_clone.clone();
            async move {
                if oauth_code.is_empty() && oauth_state.is_empty() {
                    return LoginState::None;
                }

                let credentials_repository = create_credentials_repository();

                let stored_state = credentials_repository.get_oauth_state().unwrap_or_default();
                if stored_state != oauth_state {
                    return LoginState::Error;
                }

                if !oauth_code.is_empty() {
                    let auth_repository = create_auth_repository();
                    let code_verifier = credentials_repository.get_oauth_code_verifier().unwrap();
                    let code_challenge = generate_code_challenge(&code_verifier);
                    let redirect_uri = get_redirect_uri();
                    let result = auth_repository
                        .get(&oauth_code, &redirect_uri, CLIENT_ID, &code_verifier)
                        .await;
                    return match result {
                        Ok(response) => {
                            credentials_repository.set_access_token(&response.access_token);

                            navigator.replace(Route::HomeScreen {});

                            LoginState::Success
                        }
                        Err(_) => LoginState::Error,
                    };
                }
                LoginState::None
            }
        }
    });

    let message = match login_state.read().deref() {
        None => "Please, wait...",
        Some(state) => match state {
            LoginState::None => "Continue with:",
            LoginState::Error => "Authentication failed. Please, try again.",
            LoginState::Success => "Authenticated!",
        },
    };

    let is_loading = match login_state.read().deref() {
        None => true,
        Some(LoginState::Success) => true,
        _ => false,
    };

    let auth_with_peering_db = move |_| {
        let credentials_repository = create_credentials_repository();

        let code_verifier = generate_code_verifier();
        credentials_repository.set_oauth_code_verifier(&code_verifier.clone());

        let oauth_state = generate_random_state();
        credentials_repository.set_oauth_state(&oauth_state);

        let authorized_url = build_oauth_url(&code_verifier, &oauth_state);

        web_sys::window()
            .unwrap()
            .location()
            .set_href(&authorized_url)
            .unwrap();
    };

    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen bg-gray-200",

            Image {
               class: "mb-8 filter grayscale brightness-90 contrast-125",
               src: reference::image::LOGO,
               width: 64,
               height: 64
            }

            Column {
               class: "bg-white w-[480px] border border-gray-300 rounded-lg shadow-lg p-8",
               horizontal_alignment: HorizontalAlignment::Center,
               vertical_arrangement: VerticalArrangement::Center,

               Text {
                   class: "text-gray-400 italic",
                   text: "Rotko Networks"
               }

               Text {
                   class: "text-4xl p-4",
                   text: "Peering Portal"
               }

               Text {
                   class: "p-2 text-center",
                   text: "Explore real-time peering information for the Rotko Network. Stay informed on connection status and network performance."
               }

               Text {
                   class: "p-4 font-semibold",
                   text: message
               }

               ListItem {
                   class: "w-[300px] rounded-lg border border-gray-300 bg-gray-100",
                   is_enabled: !is_loading,
                   icon: Icon {
                       width: 24,
                       height: 24,
                       src: reference::icon::OIDC.to_string()
                   },
                   label: "PeeringDB",
                   trailing_content: Some(
                        rsx!{
                            Image {
                                width: 24,
                                height: 24,
                                src: reference::icon::CHEVRON_RIGHT.to_string()
                            }
                        }
                    ),
                   on_click: auth_with_peering_db
               }
            }
        }
    }
}
