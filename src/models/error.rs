use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;


pub enum CustomError {
    BadRequest,
    NotFound,
    ServerError(String),
    OtherError(String),
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::ServerError(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
            Self::OtherError(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
            Self::BadRequest=> (StatusCode::BAD_REQUEST, "Bad Request".to_string()),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
        };
        (status, Json(json!({
            "status": "error",
            "message": error_message}))
        ).into_response()
    }
}
