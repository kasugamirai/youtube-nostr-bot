use data::db::DbConnection;
use nostr_sdk::prelude::*;
use nostr_wrapper::AsyncNotePublisher;
use nostr_wrapper::NotePublisher;
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

    let mut db_conn = match DbConnection::new("./conf/test/config.yaml") {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to create database connection: {}", e);
            return;
        }
    };

    for user_id in &config.youtube.user_id {
        let channel_id = match db_conn.query_channel_id(user_id) {
            Ok(Some(id)) => id,
            Ok(None) => {
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
            Err(e) => {
                log::error!("Failed to query channel ID: {}", e);
                continue; // Use continue to skip this iteration if we can't get the channel ID.
            }
        };

        let avatar_url = match db_conn.avatar_exists(user_id) {
            Ok(Some(url)) => url,
            Ok(None) => {
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
            Err(e) => {
                log::error!("Failed to query avatar URL: {}", e);
                continue; // Use continue to skip this iteration if we can't get the channel ID.
            }
        };

        let url = format!("https://rsshub.app/youtube/channel/{}", channel_id);
        log::info!("Channel ID: {}", channel_id);
        let fetcher = RssFetcher::new(&url);
        match fetcher.fetch().await {
            Ok(videos) => {
                for video in videos {
                    log::info!(
                        "Title: {}, Link: {}, Author: {}",
                        video.title,
                        video.link,
                        video.author_name
                    );
                    match db_conn.video_exists(&video.link) {
                        Ok(true) => {
                            log::info!("Video already exists in database");
                            continue;
                        }
                        Ok(false) => (),
                        Err(e) => {
                            log::error!("Failed to check if video exists: {}", e);
                            continue;
                        }
                    }

                    match db_conn.channel_exists(&user_id) {
                        Ok(false) => {
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
                        Ok(true) => (),
                        Err(e) => {
                            log::error!("Failed to check if user exists: {}", e);
                            continue;
                        }
                    }

                    let user_private_key_result = db_conn.find_user_private_key(&user_id);
                    let user_private_key_str: String = match user_private_key_result {
                        Ok(Some(key)) => key,
                        Ok(None) => {
                            log::error!("Failed to get user private key");
                            return;
                        }
                        Err(e) => {
                            log::error!("Failed to get user private key: {}", e);
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
                    let nostr_client = NotePublisher::new(&user_key, "./conf/test/config.yaml")
                        .await
                        .expect("Failed to create NotePublisher");
                    nostr_client.connect().await;
                    nostr_client
                        .set_metadata(&video.author_name, &avatar_url)
                        .await
                        .expect("Failed to set metadata");
                    nostr_client
                        .publish_text_note(&user_key, &format!("{}{}", &video.title, &video.link))
                        .await
                        .expect("Failed to publish text note");
                    nostr_client.disconnect().await;
                    log::info!("Published video: {}", &video.author_name);
                }
            }
            Err(e) => log::error!("Failed to fetch RSS feed: {}", e),
        }
    }
}
