use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Profile {
    pub id: u32,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
    pub verified_user: bool,
    pub verified_email: bool,
    pub networks: Vec<Network>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Network {
    pub perms: u8,
    pub asn: u32,
    pub name: String,
    pub id: u32,
}
