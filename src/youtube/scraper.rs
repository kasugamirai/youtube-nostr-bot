use reqwest;
use rss::Channel;

pub struct RssFetcher {
    pub url: String,
}

impl RssFetcher {

    pub fn new(url: &str) -> RssFetcher {
        RssFetcher {
            url: url.to_string(),
        }
    }

    pub async fn fetch(&self) -> Result<Channel, Box<dyn std::error::Error>> {
        let content = reqwest::get(&self.url).await?.text().await?;
        let channel = content.parse::<Channel>()?;
        Ok(channel)
    }
}
