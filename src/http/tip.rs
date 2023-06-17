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
        tip::{
            Tip,
            NewTip,
        },
        error::CustomError
    }
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/tips",
            routing::get(read_all)
        )
        .route("/api/v1/tips",
            routing::post(create)
        )
        .route("/api/v1/tips/:id",
            routing::get(read)
        )
        .route("/api/v1/tips",
            routing::put(update)
        )
        .route("/api/v1/tips",
            routing::delete(delete)
        )
        .route("/api/v1/tips/first",
            routing::get(first_tip)
        )
}

async fn create(
    State(app_state): State<Arc<AppState>>,
    Json(new_tip): Json<NewTip>,
) -> Result<impl IntoResponse, CustomError>{
    let tip = Tip::create(&app_state.pool, new_tip).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(tip).unwrap())).into_response())
}

async fn read(
    State(app_state): State<Arc<AppState>>,
    Path(tip_id): Path<i64>,
) -> Result<impl IntoResponse, CustomError>{
    let tip = Tip::read(&app_state.pool, tip_id).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(tip).unwrap())).into_response())
}

async fn read_all(
    State(app_state): State<Arc<AppState>>
) -> Result<impl IntoResponse, CustomError>{
    let tips = Tip::read_all(&app_state.pool).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(tips).unwrap())).into_response())
}

async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(tip): Json<Tip>,
) -> Result<impl IntoResponse, CustomError>{
    let tip = Tip::update(&app_state.pool, tip).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(tip).unwrap())).into_response())
}

async fn delete(
    State(app_state): State<Arc<AppState>>,
    Path(tip_id): Path<i64>,
) -> Result<impl IntoResponse, CustomError>{
    let tip = Tip::delete(&app_state.pool, tip_id).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(tip).unwrap())).into_response())
}

async fn first_tip(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError>{
    match Tip::read_not_published(&app_state.pool).await?{
        Some(tip) => Ok((StatusCode::OK, Json(serde_json::to_value(tip).unwrap())).into_response()),
        None => Err(CustomError::NotFound),
    }
}
