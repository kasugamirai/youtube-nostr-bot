pub mod publish;
use sonic_rs::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub nostr: Nostr,
}

#[derive(Debug, Deserialize)]
pub struct Nostr {
    pub relays: Vec<String>,
}
