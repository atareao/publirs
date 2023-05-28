use tokio;
use std::env;
use std::{path::Path, str::FromStr};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt
};
use tracing::{debug, info};
use sqlx::{
    Sqlite,
    sqlite::SqlitePoolOptions,
    migrate::{Migrator, MigrateDatabase}
};
use dotenv::dotenv;

mod http;
mod models;


#[tokio::main]
async fn main(){
    dotenv().ok();
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_|"DEBUG".to_string());
    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Log level: {}", &log_level);
    let db_url = env::var("DB_URL").unwrap_or_else(|_|"publirs.db".to_string());
    info!("Database URL: {}", &db_url);
    let port = env::var("PORT").unwrap_or_else(|_|"8080".to_string()).parse::<u16>().unwrap();
    info!("Port: {}", &port);
    let enviroment = env::var("ENVIRONMENT").unwrap_or_else(|_|"DEVELOPMENT".to_string());
    info!("Environment: {}", &port);
    let token = env::var("TOKEN").expect("TOKEN is mandatory");
    debug!("Environment: {}", &token);

    if !Sqlite::database_exists(&db_url).await.unwrap(){
        Sqlite::create_database(&db_url).await.unwrap();
    }

    let migrations = if enviroment == "PRODUCTION"{
        tracing::info!("PRODUCTION");
        std::env::current_exe().unwrap().parent().unwrap().join("migrations")
    }else{
        tracing::info!("DEVELOPMENT");
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir).join("./migrations")
    };
    debug!("Migrations: {:?}", migrations);

    let pool = match SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await{
            Ok(pool) => {
                tracing::info!("✅Connection to the database is successful!");
                pool
            },
            Err(err) => {
                tracing::info!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };
    Migrator::new(migrations)
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();

    tracing::info!("🚀 Server started successfully");
    http::serve(&pool, &token, port).await.unwrap();
}
