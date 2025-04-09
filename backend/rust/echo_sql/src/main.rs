use echo_sql::connection::Config;
use tokio;

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=migrations");
    let config = Config::new();
    let conn = match config.connect().await {
        Ok(conn) => conn,
        Err(err) => {
            println!("failed to connect to db: {}", err);
            return;
        }
    };
    match config.migrate(conn).await {
        Ok(_) => {
            println!("migration sucsessful");
        }
        Err(err) => {
            println!("migration failed: {}", err);
        }
    };
}
