use sonic_rs::{JsonContainerTrait, JsonValueTrait, Value};

pub struct YoutubeFetcher<'a> {
    api_key: &'a str,
    channel_name: &'a str,
    count: u32,
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    Sonic(sonic_rs::Error),
    Custom(String),
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

impl From<sonic_rs::Error> for Error {
    fn from(e: sonic_rs::Error) -> Self {
        Self::Sonic(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Sonic(e) => write!(f, "Sonic error: {}", e),
            Self::Custom(e) => write!(f, "Custom error: {}", e),
        }
    }
}

pub struct UserInfo {
    pub avatar_link: String,
    pub user_name: String,
}

pub struct VideoInfo {
    pub title: String,
    pub link: String,
    pub author_name: String,
    pub channel_id: String,
}

impl<'a> YoutubeFetcher<'a> {
    pub fn new(api_key: &'a str, channel_name: &'a str, count: u32) -> YoutubeFetcher<'a> {
        YoutubeFetcher {
            api_key: api_key,
            channel_name: channel_name,
            count: count,
        }
    }

    pub async fn get_channel_id(&self) -> Result<String, Error> {
        let channel_url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&type=channel&key={}",
            self.channel_name, self.api_key
        );
        let channel_response = reqwest::get(&channel_url).await?.text().await?;
        let channel_v: Value = sonic_rs::from_str(&channel_response)?;
        let channel_id = match channel_v["items"].get(0) {
            Some(item) => match item["snippet"]["channelId"].as_str() {
                Some(id) => id.to_string(),
                None => return Err(Error::Custom("Channel ID not found".to_string())),
            },
            None => {
                return Err(Error::Custom("Channel ID not found".to_string()));
            }
        };
        Ok(channel_id)
    }

    pub async fn get_user_info(&self) -> Result<UserInfo, Error> {
        let channel_id = self.get_channel_id().await?;
        let user_info_url = format!(
            "https://www.googleapis.com/youtube/v3/channels?part=snippet&id={}&key={}",
            channel_id, self.api_key
        );
        let user_info_response = reqwest::get(&user_info_url).await?.text().await?;
        let user_info_v: Value = sonic_rs::from_str(&user_info_response)?;

        let (avatar_link, user_name) = match user_info_v["items"].get(0) {
            Some(item) => {
                let avatar_link = match item["snippet"]["thumbnails"]["default"]["url"].as_str() {
                    Some(url) => url.to_string(),
                    None => return Err(Error::Custom("Avatar URL not found".to_string())),
                };
                let user_name = match item["snippet"]["title"].as_str() {
                    Some(name) => name.to_string(),
                    None => return Err(Error::Custom("User name not found".to_string())),
                };
                (avatar_link, user_name)
            }
            None => {
                return Err(Error::Custom("User info not found".to_string()));
            }
        };
        Ok(UserInfo {
            avatar_link: avatar_link,
            user_name: user_name,
        })
    }

    pub async fn fetch(&self) -> Result<Vec<VideoInfo>, Error> {
        let channel_id = self.get_channel_id().await?;
        let video_url = format!("https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet,id&order=date&maxResults={}", self.api_key, channel_id, self.count);
        let video_response = reqwest::get(&video_url).await?.text().await?;
        let video_v: Value = sonic_rs::from_str(&video_response)?;

        let items = match video_v["items"].as_array() {
            Some(items) => items,
            None => {
                return Err(Error::Custom("No items found".to_string()));
            }
        };

        let videos = items
            .iter()
            .map(|item| VideoInfo {
                title: item["snippet"]["title"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                link: format!(
                    "https://www.youtube.com/watch?v={}",
                    item["id"]["videoId"].as_str().unwrap_or_default()
                ),
                author_name: item["snippet"]["channelTitle"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                channel_id: channel_id.clone(),
            })
            .collect();

        Ok(videos)
    }
}
