use envy;
use serde::Deserialize;

use account_v2;

#[derive(Deserialize, Debug)]
struct Config {
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = envy::from_env::<Config>().unwrap();

    account_v2::start(config.port).await
}
