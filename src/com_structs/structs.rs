use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DetailEntry {
    pub client_ip: String,
    pub s_uri: String,
    pub timestamp: String,
}

impl DetailEntry {
    pub fn new(client_ip: String, s_uri: String, timestamp: String) -> Self {
        Self { client_ip, s_uri, timestamp }
    }
}

#[derive(Deserialize)]
pub struct Shorten {
    pub r_uri: String
}