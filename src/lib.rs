pub mod api_fetch;
use sonic_rs::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub youtube: YoutubeConfig,
}

#[derive(Debug, Deserialize)]
pub struct YoutubeConfig {
    pub api_key: String,
    pub user_id: Vec<String>,
    pub count: u32,
}
