use data::db::DbConnection;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut db_conn = match DbConnection::new("./conf/test/config.yaml") {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to create database connection: {}", e);
            return;
        }
    };

    db_conn.run_migrations().expect("Failed to run migrations");
}
