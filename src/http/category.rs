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
    models::{category::{
        Category,
        NewCategory}, error::CustomError}};

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
        .route("/api/v1/channels",
            routing::delete(delete)
        )
}

async fn read(
    State(app_state): State<Arc<AppState>>,
    Path(category_id): Path<i64>,
) -> impl IntoResponse{
    let category = Category::read(&app_state.pool, category_id).await
        .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            }).unwrap();
    match category{
        Some(channel) => (StatusCode::OK, Json(serde_json::to_value(channel).unwrap())).into_response(),
        None => CustomError::NotFound.into_response(),
    }
}

async fn read_all(
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    match Category::read_all(&app_state.pool).await{
        Ok(categories) => (StatusCode::OK, Json(serde_json::to_value(categories).unwrap())),
        Err(e)  => get_error(StatusCode::NOT_FOUND, format!("Error: {}", e)),
    }
}


fn get_error(status_code: StatusCode, error: String) -> (StatusCode, Json<serde_json::Value>){
    let status = if status_code == StatusCode::OK{
        "ok"
    }else{
        "ko"
    };
    let error_response = serde_json::json!({
        "status": status,
        "message": format!("Error: {}", error),
    });
    (StatusCode::NOT_FOUND, Json(error_response))
}

async fn create(
    State(app_state): State<Arc<AppState>>,
    Json(new_channel): Json<NewCategory>,
) -> impl IntoResponse{
    tracing::info!("Por aquí");
    match Category::create(&app_state.pool, new_channel).await{
        Ok(channel) => (StatusCode::OK, Json(channel)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            YTPError::NotFound.into_response()
        }
    }
}

async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(channel): Json<Category>,
) -> impl IntoResponse{
    match Category::update(&app_state.pool, channel).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(_) => YTPError::NotFound.into_response()
    }
}

async fn delete(
    State(app_state): State<Arc<AppState>>,
    Path(channel_id): Path<i64>,
) -> impl IntoResponse{
    match Category::delete(&app_state.pool, channel_id).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(_) => YTPError::NotFound.into_response()
    }
}

