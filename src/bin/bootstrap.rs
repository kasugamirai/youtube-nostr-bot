use data::db::DbConnection;
use log;
use nostr_sdk::prelude::*;
use nostr_wrapper::publish_text_note;
use std::fs::File;
use std::io::BufReader;
use youtube_bot::api_fetch::RssFetcher;
use youtube_bot::api_fetch::YoutubeFetcher;
use youtube_bot::Config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let file = File::open("./conf/test/config.yaml").expect("Failed to open config file");

    let reader = BufReader::new(file);

    let config: Config = serde_yaml::from_reader(reader).expect("Failed to read config");

    let mut db_conn = DbConnection::new();

    for user_id in &config.youtube.user_id {
        let channel_id = match db_conn.query_channel_id(user_id) {
            Some(id) => id,
            None => {
                log::info!("Channel ID not found in database. Fetching...");
                let fetcher =
                    YoutubeFetcher::new(&config.youtube.api_key, &user_id, config.youtube.count);
                match fetcher.get_channel_id().await {
                    Ok(id) => id,
                    Err(e) => {
                        log::error!("Failed to get channel ID: {}", e);
                        continue; // Use continue to skip this iteration if we can't get the channel ID.
                    }
                }
            }
        };

        let avatar_url = match db_conn.avatar_exists(user_id) {
            Some(url) => url,
            None => {
                log::info!("Avatar URL not found in database. Fetching...");
                let fetcher =
                    YoutubeFetcher::new(&config.youtube.api_key, &user_id, config.youtube.count);
                match fetcher.get_user_avatar().await {
                    Ok(url) => url,
                    Err(e) => {
                        log::error!("Failed to get avatar URL: {}", e);
                        continue; // Use continue to skip this iteration if we can't get the channel ID.
                    }
                }
            }
        };

        let url = format!("https://rsshub.app/youtube/channel/{}", channel_id);
        log::info!("Channel ID: {}", channel_id);
        let fetcher = RssFetcher::new(&url);
        let mut db_conn = DbConnection::new();
        match fetcher.fetch().await {
            Ok(videos) => {
                for video in videos {
                    log::info!(
                        "Title: {}, Link: {}, Author: {}",
                        video.title,
                        video.link,
                        video.author_name
                    );
                    if db_conn.video_exists(&video.link) {
                        log::info!("Video already exists in database");
                        continue;
                    }

                    if db_conn.channel_exists(&user_id) == false {
                        // Create a new user
                        let my_keys: Keys = Keys::generate();
                        let pk: String = match my_keys.public_key().to_bech32() {
                            Ok(pk) => pk,
                            Err(e) => {
                                log::error!("Failed to convert public key to bech32: {}", e);
                                continue;
                            }
                        };

                        let prk: String = match my_keys.secret_key() {
                            Ok(secret_key) => secret_key.to_bech32().unwrap_or_else(|_| {
                                log::error!("Failed to convert secret key to Bech32 format.");
                                String::new()
                            }),
                            Err(e) => {
                                log::error!("Failed to get secret key: {}", e);
                                String::new()
                            }
                        };

                        if let Err(e) = db_conn.add_user(
                            video.author_name.clone(),
                            avatar_url.clone(),
                            pk,
                            prk,
                            user_id.clone(),
                            channel_id.clone(),
                        ) {
                            log::error!("Failed to add user: {}", e);
                        }
                    }

                    let user_private_key_result = db_conn.find_user_private_key(&user_id);
                    let user_private_key_str: String = match user_private_key_result {
                        Some(key) => key,
                        None => {
                            log::error!("Failed to get user private key");
                            return;
                        }
                    };

                    let user_key: Keys = match Keys::from_sk_str(&user_private_key_str) {
                        Ok(keys) => keys,
                        Err(e) => {
                            log::error!("Failed to create Keys from private key: {}", e);
                            continue;
                        }
                    };

                    if let Err(e) = db_conn.add_video(
                        video.author_name.clone(),
                        user_id.clone(),
                        video.title.clone(),
                        video.link.clone(),
                        false,
                    ) {
                        log::error!("Failed to add video: {}", e);
                    }

                    let _ = publish_text_note(
                        &user_key,
                        &video.author_name.clone(),
                        &avatar_url,
                        &format!("{}{}", &video.title, &video.link),
                    )
                    .await;
                    log::info!("Published video: {}", &video.author_name);
                }
            }
            Err(e) => log::error!("Failed to fetch RSS feed: {}", e),
        }
    }
}
