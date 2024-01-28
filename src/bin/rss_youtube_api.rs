use youtube_bot::youtube::rss_fetch::RssFetcher;
use youtube_bot::youtube::youtube_fetch::YoutubeFetcher;
use nostr_wrapper::publish;
use youtube_bd::db::DbConnection;
use nostr_sdk::prelude::*;
use std::fs::File;
use std::io::BufReader;
use youtube_bot::Config;

#[tokio::main]
async fn main() {
    let file = File::open("./conf/test/config.yaml").expect("Failed to open config file");

    let reader = BufReader::new(file);

    let config: Config = serde_yaml::from_reader(reader).expect("Failed to read config");

    for user_id in &config.youtube.user_id {

        let fetcher = YoutubeFetcher::new(&config.youtube.api_key, &user_id, config.youtube.count);

        let channel_id = fetcher.get_channel_id().await.unwrap();

        let url = format!("https://rsshub.app/youtube/channel/{}", channel_id);
        println!("Channel ID: {}", channel_id);
        let fetcher = RssFetcher::new(&url);
        let mut db_conn = DbConnection::new();
        match fetcher.fetch().await {
            Ok(videos) => {
                for video in videos {
                    println!("Title: {}, Link: {}, Author: {}", video.title, video.link, video.author_name);
                    if db_conn.video_exists(&video.link) {
                        println!("Video already exists in database");
                        continue;
                    }

                    if db_conn.channel_exists(&user_id) == false {
                        // Create a new user
                        let my_keys: Keys = Keys::generate();
                        let pk: String = match my_keys.public_key().to_bech32() {
                            Ok(pk) => pk,
                            Err(e) => {
                                eprintln!("Failed to convert public key to bech32: {}", e);
                                continue;
                            }
                        };

                        let prk: String = match my_keys.secret_key() {
                            Ok(secret_key) => secret_key.to_bech32().unwrap_or_else(|_| {
                                eprintln!("Failed to convert secret key to Bech32 format.");
                                String::new() 
                            }),
                            Err(e) => {
                                eprintln!("Failed to get secret key: {}", e);
                                String::new() 
                            }
                        };
                        
                        if let Err(e) = db_conn.add_user(video.author_name.clone(), pk, prk, user_id.clone()) {
                            eprintln!("Failed to add user: {}", e);
                        }                
                    }
                    
                    let user_private_key_result = db_conn.find_user_private_key(&user_id);
                    let user_private_key_str: String = match user_private_key_result {
                        Some(key) => key,
                        None => {
                            eprintln!("Failed to get user private key");
                            return;
                        }
                    };

                    let user_key: Keys = match Keys::from_sk_str(&user_private_key_str) {
                        Ok(keys) => keys,
                        Err(e) => {
                            eprintln!("Failed to create Keys from private key: {}", e);
                            continue;
                        }
                    };

                    if let Err(e) = db_conn.add_video(video.author_name.clone(),user_id.clone(), video.title.clone(), video.link.clone(), false) {
                        eprintln!("Failed to add video: {}", e);
                    }

                    let _ = publish::publish_text_note(user_key,&video.author_name.clone(), &format!("{}{}", &video.title, &video.link)).await;
                    println!("Published video: {}", &video.author_name);

                }
            }
            Err(e) => eprintln!("Failed to fetch RSS feed: {}", e),
        }
    }
}
