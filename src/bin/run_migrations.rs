use data::db::DbConnection;

#[tokio::main]
async fn main() {
    env_logger::init();
    let dsn = "./conf/test/config.yaml";
    let mut db_conn = match DbConnection::new(dsn) {
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
