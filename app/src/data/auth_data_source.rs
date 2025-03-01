use crate::data::auth::AuthResponse;
use reqwest::Client;
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
        let url = "https://www.peeringdb.com/oauth2/token".to_string();

        let params = [
            ("grant_type", "authorization_code"),
            ("code", authorization_code),
            ("redirect_uri", redirect_uri),
            ("client_id", client_id),
            ("code_verifier", code_verifier),
        ];

        let response = self.client.post(&url).form(&params).send().await?;

        if response.status().is_success() {
            let token_response = response.json::<AuthResponse>().await?;
            Ok(token_response)
        } else {
            Err(format!("Failed to exchange token: HTTP {}", response.status()).into())
        }
    }
}
