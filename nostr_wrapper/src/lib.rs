mod publish;
pub use publish::publish_text_note;
use sonic_rs::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    nostr: Nostr,
}

#[derive(Debug, Deserialize)]
struct Nostr {
    relays: Vec<String>,
}
