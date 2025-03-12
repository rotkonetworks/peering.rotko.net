use crate::data::auth::AuthResponse;
use crate::data::create_client;
use dioxus::logger::tracing::{info, Level};
use dioxus::prelude::*;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::error::Error;

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
        code_verifier: &str,
    ) -> Result<AuthResponse, Box<dyn Error>> {
        let url = "https://auth.peeringdb.com/oauth2/token/".to_string();

        let params = [
            ("grant_type", "authorization_code"),
            ("code", &authorization_code),
            ("redirect_uri", &redirect_uri),
            ("client_id", &client_id),
            ("code_verifier", &code_verifier),
        ];

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response = response.json::<AuthResponse>().await?;
            Ok(token_response)
        } else {
            Err(format!("Failed to exchange token: HTTP {}", response.status()).into())
        }
    }

    pub async fn get2(
        &self,
        authorization_code: &str,
        redirect_uri: &str,
        client_id: &str,
    ) -> Result<AuthResponse, Box<dyn Error>> {
        get_peering_db_token(
            authorization_code.into(),
            redirect_uri.into(),
            client_id.into(),
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
) -> Result<AuthResponse, ServerFnError> {
    let client = Client::builder().build().unwrap();

    let client_secret = env::var("PEERINGDB_CLIENT_SECRET")
        .expect("Missing PEERINGDB_CLIENT_SECRET env var")
        .trim()
        .to_string();

    let masked = mask_secret(&client_secret);
    info!("Client ID: {}", &client_id);
    info!("Client Secret: {}", masked);
    info!("Code: {}", &authorization_code);
    info!("Redirect URI: {}", &redirect_uri);

    let url = "https://auth.peeringdb.com/oauth2/token/".to_string();

    let params = [
        ("grant_type", "authorization_code"),
        ("code", &authorization_code),
        ("redirect_uri", &redirect_uri),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ];

    let response = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let token_response = response.json::<AuthResponse>().await?;
        Ok(token_response)
    } else {
        let status = &response.status();
        let error_text = &response.text().await?; // Await first
        Err(ServerFnError::ServerError(format!(
            "Failed to exchange token: HTTP {}. {}",
            status, error_text
        )))
    }
}

fn mask_secret(secret: &str) -> String {
    if secret.len() < 8 {
        return "*".repeat(secret.len()); // Fully mask if too short
    }
    let (start, end) = secret.split_at(4);
    let last4 = &end[end.len().saturating_sub(4)..]; // Get last 4 safely
    format!("{}****{}", start, last4)
}
