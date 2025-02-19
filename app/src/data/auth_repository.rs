use crate::data::auth::AuthResponse;
use crate::data::auth_data_source::AuthDataSource;
use std::error::Error;

pub struct AuthRepository {
    data_source: AuthDataSource,
}

impl AuthRepository {
    pub fn new(data_source: AuthDataSource) -> Self {
        Self { data_source }
    }

    pub async fn get(
        &self,
        authorization_code: &str,
        redirect_uri: &str,
        client_id: &str,
        code_verifier: &str,
    ) -> Result<AuthResponse, Box<dyn Error>> {
        self.data_source
            .get(authorization_code, redirect_uri, client_id, code_verifier)
            .await
    }
}
