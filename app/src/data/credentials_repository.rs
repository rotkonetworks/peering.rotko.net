use crate::data::credentials_data_source::CredentialsDataSource;

pub struct CredentialsRepository {
    local_data_source: CredentialsDataSource,
}

impl CredentialsRepository {
    pub fn new(local_data_source: CredentialsDataSource) -> Self {
        Self { local_data_source }
    }

    pub fn get_oauth_state(&self) -> Option<String> {
        self.local_data_source.get_oauth_state()
    }

    pub fn set_oauth_state(&self, value: &str) {
        let _ = self.local_data_source.set_oauth_state(value);
    }

    pub fn get_oauth_code_verifier(&self) -> Option<String> {
        self.local_data_source.get_oauth_code_verifier()
    }

    pub fn set_oauth_code_verifier(&self, value: &str) {
        let _ = self.local_data_source.set_oauth_code_verifier(value);
    }

    pub fn get_access_token(&self) -> Option<String> {
        self.local_data_source.get_access_token()
    }

    pub fn set_access_token(&self, value: &str) {
        let _ = self.local_data_source.set_access_token(value);
    }
}
