use log::{error, info};
use std::collections::HashMap;
use youtube_bot::load_conf;
use youtube_bot::App;

const CONF_PATH: &str = "./conf/test/config.yaml";

#[tokio::main]
async fn main() {
    env_logger::init();

    let conf = match load_conf(CONF_PATH) {
        Ok(conf) => conf,
        Err(e) => {
            error!("Failed to load config: {}", e);
            return;
        }
    };

    let relays = conf.nostr.relays;
    let users = conf.youtube.user_id;
    let api_key = conf.youtube.api_key;
    let count = conf.youtube.count;
    let dsn = conf.postgres.dsn;

    let mut apps = HashMap::new();
    let mut user_contents = HashMap::new();

    // Initialize app instances and fetch initial contents
    for user in &users {
        match App::new(&dsn) {
            Ok(mut app) => match app.check_user(user, &api_key, count).await {
                Ok(channel_id) => match app.get_contents(&channel_id, user).await {
                    Ok(contents) => {
                        apps.insert(user.clone(), app);
                        user_contents.insert(user.clone(), contents);
                    }
                    Err(e) => error!("Failed to get contents for user {}: {}", user, e),
                },
                Err(e) => error!("Failed to check user {}: {}", user, e),
            },
            Err(e) => error!("Failed to create app for user {}: {}", user, e),
        }
    }

    // Main loop to publish one post per user per iteration
    let mut done = false;
    while !done {
        done = true;
        for user in users.clone() {
            if let Some(contents) = user_contents.get_mut(&user) {
                if let Some(msg) = contents.pop() {
                    done = false;
                    if let Some(app) = apps.get_mut(&user) {
                        match app.publish(&user, &msg, &relays).await {
                            Ok(_) => info!("Successfully published for user {}", user),
                            Err(e) => error!("Failed to publish for user {}: {}", user, e),
                        }
                    }
                }
            }
        }
    }
}
