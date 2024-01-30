pub mod publish;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub nostr: Nostr,
}

#[derive(Debug, Deserialize)]
pub struct Nostr {
    pub relays: Vec<String>,
}