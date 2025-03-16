use web_sys::Storage;

const OAUTH_STATE_KEY: &str = "oauth_state";
const OAUTH_CODE_VERIFIER_KEY: &str = "oauth_code_verifier";

const ACCESS_TOKEN_KEY: &str = "access_token";
const REFRESH_TOKEN_KEY: &str = "refresh_token";

pub struct CredentialsDataSource {
    storage: Storage,
}

impl CredentialsDataSource {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub fn get_oauth_state(&self) -> Option<String> {
        self.storage.get_item(OAUTH_STATE_KEY).ok().flatten()
    }

    pub fn set_oauth_state(&self, value: &str) {
        let _ = self.storage.set_item(OAUTH_STATE_KEY, value);
    }

    pub fn get_oauth_code_verifier(&self) -> Option<String> {
        self.storage
            .get_item(OAUTH_CODE_VERIFIER_KEY)
            .ok()
            .flatten()
    }

    pub fn set_oauth_code_verifier(&self, value: &str) {
        let _ = self.storage.set_item(OAUTH_CODE_VERIFIER_KEY, value);
    }

    pub fn get_access_token(&self) -> Option<String> {
        self.storage.get_item(ACCESS_TOKEN_KEY).ok().flatten()
    }

    pub fn set_access_token(&self, value: &str) {
        let _ = self.storage.set_item(ACCESS_TOKEN_KEY, value);
    }

    pub fn delete_access_token(&self) {
        let _ = self.storage.remove_item(ACCESS_TOKEN_KEY);
    }

    pub fn get_refresh_token(&self) -> Option<String> {
        self.storage.get_item(REFRESH_TOKEN_KEY).ok().flatten()
    }

    pub fn set_refresh_token(&self, value: &str) {
        let _ = self.storage.set_item(REFRESH_TOKEN_KEY, value);
    }

    pub fn delete_refresh_token(&self) {
        let _ = self.storage.remove_item(REFRESH_TOKEN_KEY);
    }
}
