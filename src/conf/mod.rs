use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

use sonic_rs::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub youtube: YoutubeConfig,
    pub nostr: Nostr,
    pub postgres: Postgres,
}

#[derive(Debug, Deserialize)]
pub struct YoutubeConfig {
    pub api_key: String,
    pub user_id: Vec<String>,
    pub count: u32,
}

#[derive(Debug, Deserialize)]
pub struct Nostr {
    pub relays: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub dsn: String,
}

pub fn load_conf(config_path: &str) -> Result<Config, Error> {
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let conf = serde_yaml::from_reader(reader)?;
    Ok(conf)
}
