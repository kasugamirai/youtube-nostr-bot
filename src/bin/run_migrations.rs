use data::DbConnection;
use youtube_bot::load_conf;

const CONF_PATH: &str = "./conf/test/config.yaml";

#[tokio::main]
async fn main() {
    env_logger::init();
    let conf = load_conf(CONF_PATH).expect("Failed to load config");

    let dsn = conf.postgres.dsn;
    let mut db_conn = match DbConnection::new(&dsn) {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to create database connection: {}", e);
            return;
        }
    };

    match db_conn.run_migrations() {
        Ok(_) => log::info!("Migrations ran successfully"),
        Err(e) => log::error!("Failed to run migrations: {}", e),
    }
}
