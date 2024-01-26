use youtube_bot::youtube::youtube_fetch::YoutubeFetcher;
use nostr_wrapper::publish;
use youtube_bd::{db::DbConnection, schema::youtube_users::publickey};
use nostr_sdk::prelude::*;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let user_id = match env::var("USER_ID") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("USER_ID not set");
            return;
        }
    };
    let api_key = match env::var("API_KEY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("API_KEY not set");
            return;
        }
    };
    println!("User ID: {}", user_id);
    let fetcher = YoutubeFetcher::new(&api_key, &user_id);
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
        Err(e) => eprintln!("Failed to fetch videos: {}", e),
    }
}