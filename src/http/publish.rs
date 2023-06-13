use std::sync::Arc;

use axum::{
    Router,
    routing,
    extract::State,
    http::StatusCode,
    response::IntoResponse
};
use tracing::debug;

use crate::models::{
    poll::Poll,
    category::Category,
    answer::Answer,
    telegram::Telegram,
};

use super::AppState;
use tracing::info;

pub fn router() -> Router<Arc<AppState>>{
    Router::new()
        .route("/api/v1/publish_poll",
            routing::get(publish_poll)
        )
        .route("/api/v1/category",
            routing::get)
}

async fn publish_poll(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode>{
    match Poll::read_not_published(&app_state.pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
            Some(poll) => {
                debug!("Poll: {:?}", poll);
                let category = Category::read(&app_state.pool, poll.get_category_id()).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
                debug!("Category: {:?}", category);
                let answers = Answer::read_for_poll(&app_state.pool, poll.get_id()).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                debug!("Answers: {:?}", answers);
                if answers.len() == 0 {
                    return Err(StatusCode::NOT_FOUND);
                }
                let telegram = Telegram::new(&app_state.token);
                let options = answers.iter().map(|x| x.get_text()).collect();

                let correct_option_id: i64 = answers.iter().position(|x| x.get_isok() == true).unwrap().try_into().unwrap();
                match telegram.send_poll(
                    category.get_chat_id(),
                    category.get_thread_id(),
                    poll.get_question(),
                    options,
                    correct_option_id
                ).await{
                        Ok(_) => info!("Poll send"),
                        Err(e) => info!("Cant send poll. {}", e),
                    }
                Ok(StatusCode::OK)

            },
            None => Err(StatusCode::NOT_FOUND),
        }
}
