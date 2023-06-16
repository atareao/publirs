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
    models::{
        poll::{
            Poll,
            NewPoll,
        },
        error::CustomError
    }
};

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

async fn create(
    State(app_state): State<Arc<AppState>>,
    Json(new_poll): Json<NewPoll>,
) -> impl IntoResponse{
    match Poll::create(&app_state.pool, new_poll).await{
        Ok(poll) => (StatusCode::OK, Json(serde_json::to_value(poll).unwrap())).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }

}

async fn read2(
    State(app_state): State<Arc<AppState>>,
    Path(poll_id): Path<i64>,
) -> Result<impl IntoResponse, CustomError>{
    let poll =  Poll::read(&app_state.pool, poll_id).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(poll).unwrap())).into_response())
}
async fn read(
    State(app_state): State<Arc<AppState>>,
    Path(poll_id): Path<i64>,
) -> impl IntoResponse{
    match Poll::read(&app_state.pool, poll_id).await{
        Ok(poll) => (StatusCode::OK, Json(serde_json::to_value(poll).unwrap())).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn read_all(
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    match Poll::read_all(&app_state.pool).await{
        Ok(polls) => (StatusCode::OK, Json(serde_json::to_value(polls).unwrap())).into_response(),
        Err(e)  => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(channel): Json<Poll>,
) -> impl IntoResponse{
    match Poll::update(&app_state.pool, channel).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(e)  => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn delete(
    State(app_state): State<Arc<AppState>>,
    Path(channel_id): Path<i64>,
) -> impl IntoResponse{
    match Poll::delete(&app_state.pool, channel_id).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(e)  => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

