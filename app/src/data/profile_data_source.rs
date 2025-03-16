use std::env;
use crate::data::profile::{Network, Profile};
use reqwest::Client;
use std::error::Error;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::data::auth::AuthResponse;
use crate::data::auth_data_source::get_peering_db_token;

pub struct ProfileDataSource {
    client: Client
}

impl ProfileDataSource {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, access_token: String) -> Result<Profile, Box<dyn Error>> {

        // TODO Fake
        let fake_profile = Profile {
            id: 1,
            name: "Michael Kayne".to_string(),
            given_name: "".to_string(),
            family_name: "".to_string(),
            email: "michael.kayne@example.com".to_string(),
            verified_user: false,
            verified_email: false,
            networks: vec![
                Network {
                    perms: 3,
                    asn: 65001,
                    name: "AT&T".to_string(),
                    id: 101,
                },
                Network {
                    perms: 5,
                    asn: 65002,
                    name: "Verizon Communications".to_string(),
                    id: 102,
                },
                Network {
                    perms: 2,
                    asn: 65003,
                    name: "T-Mobile USA".to_string(),
                    id: 103,
                },
            ],
        };

        return Ok(fake_profile);

        get_profile(access_token.into())
            .await
            .map_err(|e| Box::<dyn Error>::from(e.to_string()))
    }
}


/// Proxy function to avoid CORS restriction
#[server]
pub async fn get_profile(
    access_token: String
) -> Result<Profile, ServerFnError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", access_token.as_str())).unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();

    let url = "https://auth.peeringdb.com/profile/v1".to_string();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let profile_response = response.json::<Profile>().await?;
        Ok(profile_response)
    } else {
        let status = &response.status();
        let error_text = &response.text().await?; // Await first
        Err(ServerFnError::ServerError(format!(
            "Failed to exchange token: HTTP {}. {}",
            status, error_text
        )))
    }
}