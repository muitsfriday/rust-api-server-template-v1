use actix_web::{web, App, HttpServer};
use anyhow::Result;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};

pub mod core;
pub mod handler;
pub mod repo;

use repo::account_mongo;


pub struct ServerConfig {
    pub port: u16,
}

pub struct DatabaseConfig<'a> {
    pub mongo_url: &'a str,
    pub mongo_dbname: &'a str,
    pub mongo_collname_account: &'a str,
}

pub async fn start<'a>(server_conf: ServerConfig, db_conf: DatabaseConfig<'a>) -> std::io::Result<()> {
    // connect mongo
    let account_coll = init_mongo_coll(
        &db_conf.mongo_url, 
        &db_conf.mongo_dbname, 
        &db_conf.mongo_collname_account
    ).await.map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::ConnectionRefused, format!("Cannot connect to mongo db, error {}", e.to_string()))
    })?;

    let account_store = account_mongo::Repo::new(account_coll);
    let account_store = web::Data::new(account_store);

    HttpServer::new(move|| {
        App::new()
            //.app_data(web::JsonConfig::default().error_handler(handler::handle_json_error))
            .app_data(account_store.clone())
            .route("/account", actix_web::Route::to(web::post(), handler::account_create::handle::<account_mongo::Repo>))
    })
    .bind(("0.0.0.0", server_conf.port))?
    .run()
    .await
}

async fn init_mongo_coll<T: Send + Sync>(url: &str, dbname: &str, collection: &str) -> Result<Collection<T>> {
    let opts = ClientOptions::parse(url).await?;
    let client = Client::with_options(opts)?;
    let db_instance = client.database(dbname);
    let coll = db_instance.collection::<T>(collection);

    client
        .database(dbname)
        .run_command(doc! {"ping": 1})
        .await?;
    println!("Connected successfully.");

    Ok(coll)
}
