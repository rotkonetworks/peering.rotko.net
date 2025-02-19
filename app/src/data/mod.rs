use crate::data::auth_data_source::AuthDataSource;
use crate::data::auth_repository::AuthRepository;
use crate::data::credentials_data_source::CredentialsDataSource;
use crate::data::credentials_repository::CredentialsRepository;
use reqwest::Client;
use web_sys::{window, Storage};

pub mod auth;
mod auth_data_source;
pub mod auth_repository;
mod credentials_data_source;
mod credentials_repository;
pub mod profile_data_source;

fn create_storage() -> Storage {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .unwrap()
}
fn create_client() -> Client {
    Client::builder().build().unwrap()
}

pub fn create_auth_repository() -> AuthRepository {
    let client = create_client();
    let data_source = AuthDataSource::new(client.clone());
    AuthRepository::new(data_source)
}

pub fn create_credentials_repository() -> CredentialsRepository {
    let storage = create_storage();
    let local_data_source = CredentialsDataSource::new(storage);
    CredentialsRepository::new(local_data_source)
}
