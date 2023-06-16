use std::sync::Arc;

use axum::{
    Router,
    Json,
    routing,
    extract::State,
    http::StatusCode,
    response::IntoResponse
};
use tracing::debug;

use crate::models::{
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
    telegram::Telegram,
    error::CustomError,
};

use super::AppState;
use tracing::info;

pub fn router() -> Router<Arc<AppState>>{
    Router::new()
        .route("/api/v1/publish_poll",
            routing::get(publish_poll)
        )
        .route("/api/v1/create_poll",
            routing::post(create_poll)
        )
}

async fn publish_poll(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError>{
    match Poll::read_not_published(&app_state.pool).await?{
        Some(poll) => {
            debug!("Poll: {:?}", poll);
            let category = Category::read(&app_state.pool, poll.get_category_id()).await?;
            let answers = Answer::read_for_poll(&app_state.pool, poll.get_id()).await?;
            tracing::debug!("Answers: {:?}", answers);
            if answers.len() == 0 {
                return Err(CustomError::NotFound);
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
        None => {
            tracing::info!("No unpublished polls");
            Err(CustomError::NotFound)
        }
    }
}
async fn create_poll(
    State(app_state): State<Arc<AppState>>,
    Json(new_pollwa): Json<NewPollWithAnswers>,
) -> Result<impl IntoResponse, CustomError>{
    let category = Category::search(
        &app_state.pool,
        &new_pollwa.category)
        .await?;
    let new_poll =  NewPoll::new(category.get_id(), new_pollwa.question);
    let poll = Poll::create( &app_state.pool, new_poll).await?;
    let mut answers = Vec::new();
    for item in new_pollwa.answers{
        let new_answer = NewAnswer::new(poll.get_id(), item.text, item.isok);
        let answer = Answer::create(&app_state.pool, new_answer).await?;
        answers.push(answer);
    }
    let pwa = PollWithAnswers::new(
        poll.get_id(),
        poll.get_category_id(),
        poll.get_question().to_string(),
        poll.get_published(),
        answers,
    );
    Ok((StatusCode::OK, Json(pwa)).into_response())
}

