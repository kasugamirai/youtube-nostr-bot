use sonic_rs::{JsonContainerTrait, JsonValueTrait, Value};

pub struct YoutubeFetcher<'a> {
    api_key: &'a str,
    user_id: &'a str,
    count: u32,
}

pub struct VideoInfo {
    pub title: String,
    pub link: String,
    pub author_name: String,
    pub channel_id: String,
}

impl<'a> YoutubeFetcher<'a> {
    pub fn new(api_key: &'a str, user_id: &'a str, count: u32) -> YoutubeFetcher<'a> {
        YoutubeFetcher {
            api_key: api_key,
            user_id: user_id,
            count: count,
        }
    }

    pub async fn get_channel_id(&self) -> Result<String, Box<dyn std::error::Error>> {
        let channel_url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&type=channel&key={}",
            self.user_id, self.api_key
        );
        let channel_response = reqwest::get(&channel_url).await?.text().await?;
        let channel_v: Value = sonic_rs::from_str(&channel_response)?;
        let channel_id = match channel_v["items"].get(0) {
            Some(item) => match item["snippet"]["channelId"].as_str() {
                Some(id) => id.to_string(),
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No channel id found",
                    )))
                }
            },
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No channel found",
                )))
            }
        };
        Ok(channel_id)
    }

    pub async fn get_user_avatar(&self) -> Result<String, Box<dyn std::error::Error>> {
        let channel_id = self.get_channel_id().await?;
        let avatar_url = format!(
            "https://www.googleapis.com/youtube/v3/channels?part=snippet&id={}&key={}",
            channel_id, self.api_key
        );
        let avatar_response = reqwest::get(&avatar_url).await?.text().await?;
        let avatar_v: Value = sonic_rs::from_str(&avatar_response)?;

        let avatar_link = match avatar_v["items"].get(0) {
            Some(item) => match item["snippet"]["thumbnails"]["default"]["url"].as_str() {
                Some(url) => url.to_string(),
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No avatar url found",
                    )))
                }
            },
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No channel found",
                )))
            }
        };
        Ok(avatar_link)
    }

    pub async fn fetch(&self) -> Result<Vec<VideoInfo>, Box<dyn std::error::Error>> {
        let channel_id = self.get_channel_id().await?;
        let video_url = format!("https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet,id&order=date&maxResults={}", self.api_key, channel_id, self.count);
        let video_response = reqwest::get(&video_url).await?.text().await?;
        let video_v: Value = sonic_rs::from_str(&video_response)?;

        let items = match video_v["items"].as_array() {
            Some(items) => items,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No items found",
                )))
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
