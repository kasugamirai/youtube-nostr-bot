use chrono::{Duration, Utc};
use core::fmt;

use nostr_sdk::types::url;
use nostr_sdk::{Client, Keys, Metadata, ToBech32};
use nostr_sdk::{EventBuilder, Url};
use rand::Rng;
use std::io::Result;

pub struct NotePublisher {
    client: Client,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    UrlParse(url::ParseError),
    Nip19(nostr_sdk::nips::nip19::Error),
    Client(nostr_sdk::client::Error),
}

impl From<nostr_sdk::nips::nip19::Error> for Error {
    fn from(e: nostr_sdk::nips::nip19::Error) -> Self {
        Self::Nip19(e)
    }
}

impl From<nostr_sdk::client::Error> for Error {
    fn from(e: nostr_sdk::client::Error) -> Self {
        Self::Client(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::UrlParse(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Io: {}", e),
            Self::UrlParse(e) => write!(f, "UrlParse: {}", e),
            Self::Nip19(e) => write!(f, "nip19: {}", e),
            Self::Client(e) => write!(f, "Client: {}", e),
        }
    }
}
impl NotePublisher {
    pub async fn new(keys: &Keys, relays: &Vec<String>) -> Result<Self> {
        let client = Client::new(keys);
        client
            .add_relays(relays.clone())
            .await
            .expect("Failed to add relays");

        Ok(Self { client })
    }

    pub async fn connect(&self) {
        self.client.connect().await;
    }

    pub async fn set_metadata(
        &self,
        username: &str,
        avatar: &str,
    ) -> std::result::Result<(), Error> {
        let metadata = Metadata::new()
            .name(username)
            .display_name(username)
            .about("Description")
            .picture(Url::parse(avatar)?)
            .banner(Url::parse(avatar)?)
            .nip05("username@example.com")
            .lud16("0")
            .custom_field("custom_field", "value");

        self.client.set_metadata(&metadata).await?;
        Ok(())
    }

    pub async fn publish_text_note(
        &self,
        my_keys: &Keys,
        message: &str,
    ) -> std::result::Result<(), Error> {
        let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
        log::info!("Bech32 PubKey: {}", bech32_pubkey);
        let time = custom_created_at();

        let builder = EventBuilder::text_note(message, []).custom_created_at(time);
        self.client.send_event_builder(builder).await?;

        Ok(())
    }

    pub async fn disconnect(&self) {
        match self.client.disconnect().await {
            Ok(_) => (),
            Err(e) => log::error!("Failed to disconnect: {}", e),
        }
    }
}

pub fn custom_created_at() -> nostr_sdk::Timestamp {
    let now = Utc::now();
    let mut rng = rand::thread_rng();
    let minutes_to_subtract: i64 = rng.gen_range(0..60);
    let new_time = now - Duration::minutes(minutes_to_subtract);

    // Convert new_time to a Unix timestamp
    let unix_timestamp: u64 = new_time.timestamp() as u64;

    // Convert unix_timestamp to nostr_sdk::Timestamp
    let nostr_timestamp = nostr_sdk::Timestamp::from(unix_timestamp);

    nostr_timestamp
}
