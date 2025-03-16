use crate::data::location::Location;
use dioxus::prelude::*;
use reqwest::Client;
use std::error::Error;

pub struct LocationDataSource {
    client: Client,
}

impl LocationDataSource {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, _access_token: String) -> Result<Vec<Location>, Box<dyn Error>> {
        // TODO Mocked response
        let mocked_locations = vec![
            Location {
                name: "AMS-IX Hong Kong".to_string(),
                rotko_ip: "192.168.1.1".to_string(),
                peer_ip: "10.0.0.1".to_string(),
                prefix_sent: "5043".to_string(),
                prefix_received: "991".to_string(),
                session_established: "Yes".to_string(),
            },
            Location {
                name: "Cloudflare".to_string(),
                rotko_ip: "192.168.2.1".to_string(),
                peer_ip: "10.0.1.1".to_string(),
                prefix_sent: "604".to_string(),
                prefix_received: "86".to_string(),
                session_established: "No".to_string(),
            },
            Location {
                name: "Other Peer".to_string(),
                rotko_ip: "192.168.3.1".to_string(),
                peer_ip: "10.0.2.1".to_string(),
                prefix_sent: "4524".to_string(),
                prefix_received: "302".to_string(),
                session_established: "Yes".to_string(),
            },
        ];

        Ok(mocked_locations)
    }
}
