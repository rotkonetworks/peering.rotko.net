use crate::data::auth::AuthResponse;
use dioxus::prelude::*;
use reqwest::Client;
use std::env;
use std::error::Error;
use crate::data::create_client;

pub struct AuthDataSource {
    client: Client,
}

impl AuthDataSource {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(
        &self,
        authorization_code: &str,
        redirect_uri: &str,
        client_id: &str,
        // code_verifier: &str,
    ) -> Result<AuthResponse, Box<dyn Error>> {
        get_peering_db_token(
            authorization_code.into(),
            redirect_uri.into(),
            client_id.into(),
            // code_verifier.into(),
        )
        .await
        .map_err(|e| Box::<dyn Error>::from(e.to_string()))
    }
}

/// Proxy function to avoid CORS restriction
#[server]
pub async fn get_peering_db_token(
    authorization_code: String,
    redirect_uri: String,
    client_id: String,
    // code_verifier: String,
) -> Result<AuthResponse, ServerFnError> {
    let client = create_client(None);

    let client_secret = env::var("PEERINGDB_CLIENT_SECRET")
        .expect("Environment variable PEERINGDB_CLIENT_SECRET is not set. Verify if the variable was correctly set, likely by the Docker and CI system.");

    let url = "https://auth.peeringdb.com/oauth2/token".to_string();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", &redirect_uri),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        // ("code_verifier", &code_verifier),
        ("code", &authorization_code),
    ];

    let response = client.post(&url).form(&params).send().await?;

    if response.status().is_success() {
        let token_response = response.json::<AuthResponse>().await?;
        Ok(token_response)
    } else {
        Err(ServerFnError::ServerError(response.json().await.unwrap()))
    }
}
