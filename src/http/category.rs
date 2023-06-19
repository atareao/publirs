use std::sync::Arc;
use axum::{
    Router,
    Json,
    extract::{State, Path},
    routing,
    response::IntoResponse,
    http::StatusCode,
};

use crate::{
    http::AppState,
    models::category::{
            Category,
            NewCategory
        }
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/categories",
            routing::get(read_all)
        )
        .route("/api/v1/categories",
            routing::post(create)
        )
        .route("/api/v1/categories/:id",
            routing::get(read)
        )
        .route("/api/v1/categories",
            routing::put(update)
        )
        .route("/api/v1/categories",
            routing::delete(delete)
        )
}

async fn read(
    State(app_state): State<Arc<AppState>>,
    Path(category_id): Path<i64>,
) -> impl IntoResponse{
    match Category::read(&app_state.pool, category_id).await{
        Ok(category) => (StatusCode::OK, Json(serde_json::to_value(category).unwrap())).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn read_all(
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    match Category::read_all(&app_state.pool).await{
        Ok(categories) => (StatusCode::OK, Json(serde_json::to_value(categories).unwrap())).into_response(),
        Err(e)  => {
            tracing::error!("Error: {:?}", e);
            e.into_response()
        },
    }
}

async fn create(
    State(app_state): State<Arc<AppState>>,
    Json(new_channel): Json<NewCategory>,
) -> impl IntoResponse{
    match Category::create(&app_state.pool, new_channel).await{
        Ok(channel) => (StatusCode::OK, Json(channel)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(channel): Json<Category>,
) -> impl IntoResponse{
    match Category::update(&app_state.pool, channel).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn delete(
    State(app_state): State<Arc<AppState>>,
    Path(channel_id): Path<i64>,
) -> impl IntoResponse{
    match Category::delete(&app_state.pool, channel_id).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(e) => {
            tracing::error!("Error: {},", e);
            e.into_response()
        }
    }
}

