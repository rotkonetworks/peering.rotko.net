use crate::data::profile::Profile;
use reqwest::Client;
use std::error::Error;

pub struct ProfileDataSource {
    client: Client,
}

impl ProfileDataSource {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Profile, Box<dyn Error>> {
        let url = "https://www.peeringdb.com/profile/v1".to_string();
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let result = response.json::<Profile>().await?;
            Ok(result)
        } else {
            Err(format!("Failed to fetch profile: HTTP {}", response.status()).into())
        }
    }
}
