#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    Serde(serde_yaml::Error),
    Rss(rss::Error),
}

impl From<rss::Error> for Error {
    fn from(e: rss::Error) -> Self {
        Self::Rss(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Serde(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Serde(e) => write!(f, "Serde error: {}", e),
            Self::Rss(e) => write!(f, "Rss error: {}", e),
        }
    }
}

pub struct RssFetcher {
    url: String,
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

    pub async fn fetch(&self) -> Result<Vec<VideoInfo>, Error> {
        let content = reqwest::get(&self.url).await?.text().await?;
        let channel = content.parse::<rss::Channel>()?;

        let videos = channel
            .items()
            .iter()
            .map(|item| VideoInfo {
                title: item.title().unwrap_or_default().to_string(),
                link: item.link().unwrap_or_default().to_string(),
                author_name: item.author().unwrap_or_default().to_string(),
            })
            .collect();

        Ok(videos)
    }
}
