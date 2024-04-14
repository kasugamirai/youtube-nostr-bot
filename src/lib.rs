mod api;
mod conf;
mod gpt;
mod nostr;

use std::str::FromStr;
use std::time::Duration;
use tokio::time;

pub use api::RssFetcher;
pub use api::YoutubeFetcher;
pub use conf::load_conf;
pub use conf::Config;
use data::DbConnection;
pub use nostr::NotePublisher;
use nostr_sdk::SecretKey;

use nostr_sdk::Keys;
use nostr_sdk::ToBech32;

#[derive(Debug)]
pub enum Error {
    DbError(data::Error),
    ConfigError(conf::Error),
    NIP19(nostr_sdk::nips::nip19::Error),
    Nostr(nostr_sdk::key::Error),
    NostrWrapper(nostr::Error),
    Rss(api::RssError),
    Youtube(api::YoutubeError),
    IO(std::io::Error),
    Custom(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<nostr::Error> for Error {
    fn from(e: nostr::Error) -> Self {
        Self::NostrWrapper(e)
    }
}

impl From<api::RssError> for Error {
    fn from(e: api::RssError) -> Self {
        Self::Rss(e)
    }
}

impl From<api::YoutubeError> for Error {
    fn from(e: api::YoutubeError) -> Self {
        Self::Youtube(e)
    }
}

impl From<nostr_sdk::key::Error> for Error {
    fn from(e: nostr_sdk::key::Error) -> Self {
        Self::Nostr(e)
    }
}

impl From<nostr_sdk::nips::nip19::Error> for Error {
    fn from(e: nostr_sdk::nips::nip19::Error) -> Self {
        Self::NIP19(e)
    }
}

impl From<data::Error> for Error {
    fn from(e: data::Error) -> Self {
        Self::DbError(e)
    }
}

impl From<conf::Error> for Error {
    fn from(e: conf::Error) -> Self {
        Self::ConfigError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DbError(e) => write!(f, "Database error: {}", e),
            Self::ConfigError(e) => write!(f, "Config error: {}", e),
            Self::NIP19(e) => write!(f, "Nip19 error: {}", e),
            Self::Custom(e) => write!(f, "Custom error: {}", e),
            Self::Nostr(e) => write!(f, "Nostr error: {}", e),
            Self::Rss(e) => write!(f, "Rss error: {}", e),
            Self::Youtube(e) => write!(f, "Youtube error: {}", e),
            Self::NostrWrapper(e) => write!(f, "NostrWrapper error: {}", e),
            Self::IO(e) => write!(f, "IO error: {}", e),
        }
    }
}

pub struct App {
    db: DbConnection,
}

impl App {
    pub fn new(dsn: &str) -> Result<Self, Error> {
        let db = DbConnection::new(dsn)?;
        Ok(Self { db })
    }

    pub async fn check_user(
        &mut self,
        channel_name: &str,
        api: &str,
        count: u32,
    ) -> Result<String, Error> {
        let user_exists = self.db.user_exists(channel_name).await?;
        if user_exists {
            let chid = match self.db.query_channel_id(channel_name).await? {
                Some(id) => id,
                None => {
                    return Err(Error::Custom("Channel ID not found".to_string()));
                }
            };
            Ok(chid)
        } else {
            let youtube = YoutubeFetcher::new(api, channel_name, count);
            let channel_id = youtube.get_channel_id().await?;
            let user_info = youtube.get_user_info().await?;
            let user_name = user_info.user_name;
            let avatar_url = user_info.avatar_link;

            let key = new_key()?;
            let private_key = key.secret_key;
            let public_key = key.public_key;
            self.db
                .add_user(
                    &user_name,
                    &avatar_url,
                    &public_key,
                    &private_key,
                    channel_name,
                    &channel_id,
                )
                .await?;
            Ok(channel_id)
        }
    }

    pub async fn get_contents(
        &mut self,
        channel_id: &str,
        channel_name: &str,
    ) -> Result<Vec<String>, Error> {
        let url = format!("https://rsshub.app/youtube/channel/{}", channel_id);
        let rss = RssFetcher::new(&url);
        let videos = rss.fetch().await?;
        let mut ret = Vec::new();

        for video in videos {
            let video_exists = self.db.video_exists(&video.link).await?;
            if !video_exists {
                self.db
                    .add_video(
                        &video.author_name,
                        channel_name,
                        &video.title,
                        &video.link,
                        false,
                    )
                    .await?;
                let combined = format!("{}: {}", video.title, video.link);
                ret.push(combined);
            }
        }
        Ok(ret)
    }

    pub async fn publish(
        &mut self,
        channel_name: &str,
        message: &str,
        relays: &[String],
    ) -> Result<(), Error> {
        let secret_key = match self.db.find_user_private_key(channel_name).await {
            Ok(Some(key)) => key,
            Ok(None) => {
                return Err(Error::Custom("User private key not found".to_string()));
            }
            Err(e) => {
                return Err(Error::DbError(e));
            }
        };

        let avatar = match self.db.query_avatar(channel_name).await {
            Ok(Some(avatar)) => avatar,
            Ok(None) => {
                return Err(Error::Custom("Avatar not found".to_string()));
            }
            Err(e) => {
                return Err(Error::DbError(e));
            }
        };

        let key = match self.convert_key(&secret_key) {
            Ok(key) => key,
            Err(e) => {
                return Err(e);
            }
        };

        let note_publish = match NotePublisher::new(&key, relays).await {
            Ok(publisher) => publisher,
            Err(e) => {
                return Err(Error::IO(e));
            }
        };

        let user_name = match self.db.query_user_name(channel_name).await {
            Ok(Some(name)) => name,
            Ok(None) => {
                return Err(Error::Custom("User name not found".to_string()));
            }
            Err(e) => {
                return Err(Error::DbError(e));
            }
        };

        note_publish.connect().await;
        if let Err(e) = note_publish.set_metadata(&user_name, &avatar).await {
            log::error!("Failed to set metadata: {}", e);
        }

        if let Err(e) = note_publish.publish_text_note(&key, message).await {
            log::error!("Failed to publish text note: {}", e);
        }
        time::sleep(Duration::from_secs(1)).await;
        note_publish.disconnect().await;

        Ok(())
    }

    fn convert_key(&self, secret_key: &str) -> Result<Keys, Error> {
        let sk = SecretKey::from_str(secret_key)?;
        let key = Keys::new(sk);
        Ok(key)
    }
}

#[derive(Clone)]
pub struct MyKey {
    pub public_key: String,
    pub secret_key: String,
}

fn new_key() -> Result<MyKey, Error> {
    let my_keys: Keys = Keys::generate();
    let pk = my_keys.public_key().to_bech32()?;
    let prk = my_keys.secret_key()?.to_bech32()?;
    let ret = MyKey {
        public_key: pk,
        secret_key: prk,
    };
    Ok(ret)
}
