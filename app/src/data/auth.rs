use serde::{Deserialize, Serialize};

/// Represents an _Auth_ response from _PeeringDB API_.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub scope: Option<String>,
}
