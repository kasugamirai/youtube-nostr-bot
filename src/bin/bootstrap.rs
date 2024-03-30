use data::db::DbConnection;
use nostr_sdk::nostr::Keys;
use nostr_sdk::SecretKey;
use nostr_sdk::ToBech32;
use nostr_wrapper::AsyncNotePublisher;
use nostr_wrapper::NotePublisher;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use youtube_bot::api_fetch::RssFetcher;
use youtube_bot::api_fetch::YoutubeFetcher;
use youtube_bot::Config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let file = match File::open("./conf/test/config.yaml") {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to open config file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);

    let config: Config = match serde_yaml::from_reader(reader) {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to read config: {}", e);
            return;
        }
    };

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
                        continue;
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to query channel ID: {}", e);
                continue;
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
                        continue;
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to query avatar URL: {}", e);
                continue;
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
                                Ok(secret_key) => match secret_key.to_bech32() {
                                    Ok(bech32) => bech32,
                                    Err(_) => {
                                        log::error!(
                                            "Failed to convert secret key to Bech32 format."
                                        );
                                        continue;
                                    }
                                },
                                Err(e) => {
                                    log::error!("Failed to get secret key: {}", e);
                                    continue;
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

                    let sk = match SecretKey::from_str(&user_private_key_str) {
                        Ok(sk) => sk,
                        Err(e) => {
                            log::error!("Failed to create Keys from private key: {}", e);
                            continue;
                        }
                    };

                    let user_key: Keys = Keys::new(sk);

                    if let Err(e) = db_conn.add_video(
                        video.author_name.clone(),
                        user_id.clone(),
                        video.title.clone(),
                        video.link.clone(),
                        false,
                    ) {
                        log::error!("Failed to add video: {}", e);
                    }
                    let nostr_client =
                        match NotePublisher::new(&user_key, "./conf/test/config.yaml").await {
                            Ok(client) => client,
                            Err(e) => {
                                log::error!("Failed to create NotePublisher: {}", e);
                                continue;
                            }
                        };
                    nostr_client.connect().await;
                    if let Err(e) = nostr_client
                        .set_metadata(&video.author_name, &avatar_url)
                        .await
                    {
                        log::error!("Failed to set metadata: {}", e);
                        continue;
                    }
                    if let Err(e) = nostr_client
                        .publish_text_note(&user_key, &format!("{}{}", &video.title, &video.link))
                        .await
                    {
                        log::error!("Failed to publish text note: {}", e);
                        continue;
                    }
                    nostr_client.disconnect().await;
                    log::info!("Published video: {}", &video.author_name);
                }
            }
            Err(e) => log::error!("Failed to fetch RSS feed: {}", e),
        }
    }
}
