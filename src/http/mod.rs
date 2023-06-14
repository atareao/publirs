mod publish;
mod category;
mod poll;

use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use axum::Server;
use sqlx::SqlitePool;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub token: String,
}

impl AppState {
    pub fn new(pool: &SqlitePool, token: &str) -> Self{
        Self {
            pool: pool.clone(),
            token: token.to_string(),
        }
    }
}

pub async fn serve(pool: &SqlitePool, token: &str, port: u16) -> anyhow::Result<()> {
    let app_state = AppState::new(pool, token);
    let app = publish::router()
        .with_state(Arc::new(app_state))
        .layer(TraceLayer::new_for_http());

    Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))
        .serve(app.into_make_service())
        .await
        .map_err(|_err| anyhow::anyhow!("Can't init")
    )
}

