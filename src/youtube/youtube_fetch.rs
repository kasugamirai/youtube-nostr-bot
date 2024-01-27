use reqwest;
use serde_json::Value;

pub struct YoutubeFetcher {
    pub api_key: String,
    pub user_id: String,
    pub count: u32,
}

pub struct VideoInfo {
    pub title: String,
    pub link: String,
    pub author_name: String,
    pub channel_id: String,
}

impl YoutubeFetcher {
    pub fn new(api_key: &str, user_id: &str, count: u32) -> YoutubeFetcher {
        YoutubeFetcher {
            api_key: api_key.to_string(),
            user_id: user_id.to_string(),
            count: count,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<VideoInfo>, Box<dyn std::error::Error>> {
        // First, get the channel ID from the user ID
        let channel_url = format!("https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&type=channel&key={}", self.user_id, self.api_key);
        let channel_response = reqwest::get(&channel_url).await?.text().await?;
        let channel_v: Value = serde_json::from_str(&channel_response)?;
        let channel_id = match channel_v["items"].get(0) {
            Some(item) => match item["snippet"]["channelId"].as_str() {
                Some(id) => id.to_string(),
                None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No channel id found"))),
            },
            None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No channel found"))),
        };
    
        // Then, get the videos from the channel ID
        let video_url = format!("https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet,id&order=date&maxResults={}", self.api_key, channel_id, self.count);
        let video_response = reqwest::get(&video_url).await?.text().await?;
        let video_v: Value = serde_json::from_str(&video_response)?;
    
        let items = match video_v["items"].as_array() {
            Some(items) => items,
            None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No items found"))),
        };
    
        let videos = items.iter().map(|item| {
            VideoInfo {
                title: item["snippet"]["title"].as_str().unwrap_or_default().to_string(),
                link: format!("https://www.youtube.com/watch?v={}", item["id"]["videoId"].as_str().unwrap_or_default()),
                author_name: item["snippet"]["channelTitle"].as_str().unwrap_or_default().to_string(),
                channel_id: channel_id.clone(),
            }
        }).collect();
    
        Ok(videos)
    }
}