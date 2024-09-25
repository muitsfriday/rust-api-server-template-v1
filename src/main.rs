use envy;
use serde::Deserialize;

use account_v2::{self, DatabaseConfig, ServerConfig};

#[derive(Deserialize, Debug)]
struct Config {
    port: u16,
    mongo_url: String,
    mongo_dbname: String,
    mongo_collection_account: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = envy::from_env::<Config>().unwrap();

    account_v2::start(ServerConfig {
        port: config.port,
    }, DatabaseConfig {
        mongo_url: config.mongo_url.as_str(),
        mongo_dbname: config.mongo_dbname.as_str(),
        mongo_collname_account: &config.mongo_collection_account.as_str(),
    }).await
}
