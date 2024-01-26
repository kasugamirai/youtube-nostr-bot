pub mod youtube;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub user_id: String,
    pub api_key: String,
}
