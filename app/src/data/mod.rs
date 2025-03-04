use crate::data::auth_data_source::AuthDataSource;
use crate::data::auth_repository::AuthRepository;
use crate::data::credentials_data_source::CredentialsDataSource;
use crate::data::credentials_repository::CredentialsRepository;
use crate::data::profile_data_source::ProfileDataSource;
use crate::data::profile_repository::ProfileRepository;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use web_sys::{window, Storage};
pub mod auth;
mod auth_data_source;
pub mod auth_repository;
mod credentials_data_source;
mod credentials_repository;
pub mod profile;
mod profile_data_source;
mod profile_repository;

fn create_storage() -> Storage {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .unwrap()
}

fn create_client(access_token: Option<String>) -> Client {
    if let Some(access_token) = access_token {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Token {}", access_token.as_str())).unwrap(),
        );
        Client::builder().default_headers(headers).build().unwrap()
    } else {
        Client::builder().build().unwrap()
    }
}

pub fn create_auth_repository() -> AuthRepository {
    let client = create_client(None);
    let data_source = AuthDataSource::new(client.clone());
    AuthRepository::new(data_source)
}

pub fn create_credentials_repository() -> CredentialsRepository {
    let storage = create_storage();
    let local_data_source = CredentialsDataSource::new(storage);
    CredentialsRepository::new(local_data_source)
}

pub fn create_profile_repository(access_token: String) -> ProfileRepository {
    let client = create_client(Some(access_token));
    let data_source = ProfileDataSource::new(client);
    ProfileRepository::new(data_source)
}