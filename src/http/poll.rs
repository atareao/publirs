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
    models::{poll::{
        Poll,
        NewPoll}, error::CustomError}};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/polls",
            routing::get(read_all)
        )
        .route("/api/v1/polls",
            routing::post(create)
        )
        .route("/api/v1/polls/:id",
            routing::get(read)
        )
        .route("/api/v1/polls",
            routing::put(update)
        )
        .route("/api/v1/channels",
            routing::delete(delete)
        )
}

async fn read(
    State(app_state): State<Arc<AppState>>,
    Path(poll_id): Path<i64>,
) -> impl IntoResponse{
    let poll = Poll::read(&app_state.pool, poll_id).await
        .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            }).unwrap();
    match poll{
        Some(channel) => (StatusCode::OK, Json(serde_json::to_value(channel).unwrap())).into_response(),
        None => CustomError::NotFound.into_response(),
    }
}

async fn read_all(
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    match Poll::read_all(&app_state.pool).await{
        Ok(polls) => (StatusCode::OK, Json(serde_json::to_value(categories).unwrap())),
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
    Json(new_channel): Json<NewPoll>,
) -> impl IntoResponse{
    tracing::info!("Por aquí");
    match Poll::create(&app_state.pool, new_channel).await{
        Ok(channel) => (StatusCode::OK, Json(channel)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            YTPError::NotFound.into_response()
        }
    }
}

async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(channel): Json<Poll>,
) -> impl IntoResponse{
    match Poll::update(&app_state.pool, channel).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(_) => YTPError::NotFound.into_response()
    }
}

async fn delete(
    State(app_state): State<Arc<AppState>>,
    Path(channel_id): Path<i64>,
) -> impl IntoResponse{
    match Poll::delete(&app_state.pool, channel_id).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(_) => YTPError::NotFound.into_response()
    }
}

