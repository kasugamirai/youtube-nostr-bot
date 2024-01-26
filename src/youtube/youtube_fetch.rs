use reqwest;
use serde_json::Value;

pub struct YoutubeFetcher {
    pub api_key: String,
    pub user_id: String,
}

pub struct VideoInfo {
    pub title: String,
    pub link: String,
    pub author_name: String,
}

impl YoutubeFetcher {
    pub fn new(api_key: &str, user_id: &str) -> YoutubeFetcher {
        YoutubeFetcher {
            api_key: api_key.to_string(),
            user_id: user_id.to_string(),
        }
    }

    pub async fn fetch(&self) -> Result<Vec<VideoInfo>, Box<dyn std::error::Error>> {
        // First, get the channel ID from the user ID
        let channel_url = format!("https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&type=channel&key={}", self.user_id, self.api_key);
        print!("Channel URL: {}", channel_url);
        let channel_response = reqwest::get(&channel_url).await?.text().await?;
        print!("Channel response: {}", channel_response);
        let channel_v: Value = serde_json::from_str(&channel_response)?;
        let channel_id = channel_v["items"][0]["snippet"]["channelId"].as_str().unwrap_or_default();
    
        // Then, get the videos from the channel ID
        let video_url = format!("https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet,id&order=date&maxResults=100h", self.api_key, channel_id);
        let video_response = reqwest::get(&video_url).await?.text().await?;
        let video_v: Value = serde_json::from_str(&video_response)?;
    
        let videos = video_v["items"].as_array().unwrap().iter().map(|item| {
            VideoInfo {
                title: item["snippet"]["title"].as_str().unwrap_or_default().to_string(),
                link: format!("https://www.youtube.com/watch?v={}", item["id"]["videoId"].as_str().unwrap_or_default()),
                author_name: item["snippet"]["channelTitle"].as_str().unwrap_or_default().to_string(),
            }
        }).collect();
    
        Ok(videos)
    }
}