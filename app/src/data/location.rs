use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Location {
    pub name: String,
    pub rotko_ip: String,
    pub peer_ip: String,
    pub prefix_sent: String,
    pub prefix_received: String,
    pub session_established: String,
}

