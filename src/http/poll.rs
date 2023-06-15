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
        category::Category,
        poll::{
            Poll,
            NewPoll,
            NewPollWithAnswers,
            PollWithAnswers
        },
        answer::{
            Answer,
            NewAnswer
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
        Ok(polls) => (StatusCode::OK, Json(serde_json::to_value(polls).unwrap())),
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
    Json(new_pollwa): Json<NewPollWithAnswers>,
) -> impl IntoResponse{
    tracing::info!("Por aquí");
    let category_id = match Category::search(
        &app_state.pool,
        &new_pollwa.category)
        .await{
            Ok(category) => category.get_id(),
            Err(e) => return e.into_response(),
        };
    let new_poll =  NewPoll {
        category_id,
        question: new_pollwa.question
    };
    let poll = match Poll::create(
        &app_state.pool,
        new_poll)
    .await{
            Ok(poll) => poll,
            Err(e) => return e.into_response(),
    };
    let mut answers = Vec::new();
    for item in new_pollwa.answers{
        let new_answer = NewAnswer{
            poll_id: poll.get_id(),
            text: item.text,
            isok: item.isok
        };
        let answer = match Answer::create(&app_state.pool, new_answer).await{
            Ok(answer) => answer,
            Err(e) => return e.into_response(),
        };
        answers.push(answer);
    }
    let pwa = PollWithAnswers{
        id: poll.get_id(),
        category_id: poll.get_category_id(),
        question: poll.get_question().to_string(),
        published: poll.get_published(),
        answers,
    };
    (StatusCode::OK, Json(pwa)).into_response()
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

