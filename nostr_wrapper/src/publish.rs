use crate::Config;
use async_trait::async_trait;
use nostr_sdk::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Result};

pub struct NotePublisher {
    client: Client,
}

#[async_trait]
pub trait AsyncNotePublisher {
    async fn new(keys: &Keys, config_path: &str) -> Result<Self>
    where
        Self: Sized;

    async fn connect(&self);

    async fn set_metadata(
        &self,
        username: &str,
        avatar: &str,
    ) -> std::result::Result<(), Box<dyn Error>>;

    async fn publish_text_note(
        &self,
        my_keys: &Keys,
        message: &str,
    ) -> std::result::Result<(), Box<dyn Error>>;

    async fn disconnect(&self);
}

#[async_trait]
impl AsyncNotePublisher for NotePublisher {
    async fn new(keys: &Keys, config_path: &str) -> Result<Self> {
        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_yaml::from_reader(reader).expect("Failed to read config");

        let client = Client::new(keys);
        client
            .add_relays(config.nostr.relays)
            .await
            .expect("Failed to add relays");

        Ok(Self { client })
    }

    async fn connect(&self) {
        self.client.connect().await;
    }

    async fn set_metadata(
        &self,
        username: &str,
        avatar: &str,
    ) -> std::result::Result<(), Box<dyn Error>> {
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

    async fn publish_text_note(
        &self,
        my_keys: &Keys,
        message: &str,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
        log::info!("Bech32 PubKey: {}", bech32_pubkey);

        self.client.publish_text_note(message, []).await?;
        Ok(())
    }

    async fn disconnect(&self) {
        match self.client.disconnect().await {
            Ok(_) => (),
            Err(e) => log::error!("Failed to disconnect: {}", e),
        }
    }
}
