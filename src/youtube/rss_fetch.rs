use reqwest;
use rss;

pub struct RssFetcher {
    pub url: String,
}

pub struct VideoInfo {
    pub title: String,
    pub link: String,
    pub author_name: String,
}


impl RssFetcher {

    pub fn new(url: &str) -> RssFetcher {
        RssFetcher {
            url: url.to_string(),
        }
    }

    pub async fn fetch(&self) -> Result<Vec<VideoInfo>, Box<dyn std::error::Error>> {
        let content = reqwest::get(&self.url).await?.text().await?;
        let channel = content.parse::<rss::Channel>()?;

        let videos = channel.items().iter().map(|item| {
            VideoInfo {
                title: item.title().unwrap_or_default().to_string(),
                link: item.link().unwrap_or_default().to_string(),
                author_name: item.author().unwrap_or_default().to_string(),
            }
        }).collect();

        Ok(videos)
    }

}
