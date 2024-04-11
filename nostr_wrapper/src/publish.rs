use core::fmt;
use nostr_sdk::Url;
use nostr_sdk::{Client, Keys, Metadata, ToBech32};
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
            .lud16("yuki@getalby.com")
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

        self.client.publish_text_note(message, []).await?;
        Ok(())
    }

    pub async fn disconnect(&self) {
        match self.client.disconnect().await {
            Ok(_) => (),
            Err(e) => log::error!("Failed to disconnect: {}", e),
        }
    }
}
