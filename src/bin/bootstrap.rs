use youtube_bot::youtube::scraper::RssFetcher;
use nostr_wrapper::publish;
use youtube_bd::{db::DbConnection, schema::youtube_users::publickey};
use nostr_sdk::prelude::*;
use std::env;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let channel_id = match env::var("CHANNEL_ID") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("CHANNEL_ID not set");
            return;
        }
    };
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

                if db_conn.channel_exists(&video.author_name) == false {
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
                            String::new() // or any default String value
                        }),
                        Err(e) => {
                            eprintln!("Failed to get secret key: {}", e);
                            String::new() // or any default String value
                        }
                    };
                    
                    db_conn.add_user(video.author_name.clone(), pk, prk, channel_id.clone());
                }
                let user_private_key_result = db_conn.find_user_private_key(&channel_id);
                let user_private_key_str = user_private_key_result.unwrap();


                let userKey: Keys = match Keys::from_sk_str(&user_private_key_str) {
                    Ok(keys) => keys,
                    Err(e) => {
                        eprintln!("Failed to create Keys from private key: {}", e);
                        continue;
                    }
                };
                db_conn.add_video(video.author_name.clone(),channel_id.clone(), video.title.clone(), video.link.clone(), false);
                let _ = publish::publish_text_note(userKey,&video.author_name.clone(), &format!("{}{}", &video.title, &video.link)).await;
                println!("Published video: {}", &video.author_name);

            }
        }
        Err(e) => eprintln!("Failed to fetch RSS feed: {}", e),
    }
}
