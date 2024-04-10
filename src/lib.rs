mod api_fetch;
mod conf;

pub use api_fetch::RssFetcher;
pub use api_fetch::YoutubeFetcher;
pub use conf::load_conf;
pub use conf::Config;
use data::db::DbConnection;

#[derive(Debug)]
pub enum Error {
    DbError(data::Error),
    ConfigError(conf::Error),
}

impl From<data::db::Error> for Error {
    fn from(e: data::db::Error) -> Self {
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
        }
    }
}

pub struct App {
    db: DbConnection,
}

impl App {
    pub fn new(config_path: &str) -> Result<Self, Error> {
        let conf = load_conf(config_path)?;
        let db = DbConnection::new(&conf.postgres.dsn)?;
        Ok(Self { db })
    }

    pub fn run(&mut self) {
        let rss_fetcher = RssFetcher::new();
        let youtube_fetcher = YoutubeFetcher::new();
        let config = conf::load_conf("config.yaml").unwrap();
        println!("{:?}", config);
    }
}
