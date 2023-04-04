use actix_web::{web, App, HttpServer};

pub mod core;
pub mod handler;

use handler::account_create;

pub async fn start(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/account", web::post().to(account_create::handle)))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
