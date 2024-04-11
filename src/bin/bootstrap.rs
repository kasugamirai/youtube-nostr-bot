use youtube_bot::load_conf;
use youtube_bot::App;

const CONF_PATH: &str = "./conf/test/config.yaml";

#[tokio::main]
async fn main() {
    env_logger::init();
    let conf = load_conf(CONF_PATH).expect("Failed to load config");
    let relays = conf.nostr.relays;
    let users = conf.youtube.user_id;
    let api_key = conf.youtube.api_key;
    let count = conf.youtube.count;
    let dsn = conf.postgres.dsn;
    for user in users {
        let mut app = App::new(&dsn).expect("Failed to create app");
        let channel_id = app
            .check_user(&user, &api_key, count)
            .await
            .expect("Failed to check user");
        let contents = app
            .get_contents(channel_id)
            .await
            .expect("Failed to get contents");
        for msg in &contents {
            let _ = app
                .publish(&user, msg, &relays)
                .await
                .expect("Failed to publish");
        }
    }
}
