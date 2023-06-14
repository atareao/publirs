use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use super::{answer::{Answer, NewAnswer, NewBasicAnswer}, category::Category};
use super::error::CustomError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poll{
    id: i64,
    category_id: i64,
    question: String,
    #[serde(default = "get_default_published")]
    published: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewPoll{
    category_id: i64,
    question: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewPollWithAnswers{
    category: String,
    question: String,
    answers: Vec<NewBasicAnswer>,
}


fn get_default_published() -> bool{
    false
}

impl NewBasicAnswer{
    pub async fn create(pool: &SqlitePool, new_pollwa: NewPollWithAnswers)
            -> Result<Poll, CustomError>{
        tracing::info!("Data: {:?}", new_pollwa);
        let category_id = Category::search(pool, &new_pollwa.category)
            .await?
            .get_id();
        let new_poll =  NewPoll {
            category_id,
            question: new_pollwa.question
        };
        let poll = Poll::create(pool, new_poll).await?;
        for item in new_pollwa.answers{
            let new_answer = NewAnswer{
                poll_id: poll.get_id(),
                text: item.text,
                isok: item.isok
            };
            Answer::create(&pool, new_answer).await?;
        }
        Ok(poll)
    }

}

impl Poll{
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            category_id: row.get("category_id"),
            question: row.get("question"),
            published: row.get("published")
        }
    }

    pub fn get_id(&self) -> i64{
        self.id
    }

    pub fn get_category_id(&self) -> i64{
        self.category_id
    }

    pub fn get_question(&self) -> &str{
        &self.question
    }

    pub fn get_published(&self) -> bool{
        self.published
    }

    pub async fn create(pool: &SqlitePool, new_poll: NewPoll)
            -> Result<Poll, CustomError>{
        tracing::info!("Data: {:?}", new_poll);
        let sql = "INSERT INTO polls (category_id, question
                   VALUES ($1, $2) RETURNING *;";
        query(sql)
            .bind(new_poll.category_id)
            .bind(new_poll.question)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                CustomError::ServerError(e.to_string())
            })
    }
    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Option<Poll>, sqlx::Error>{
        let sql = "SELECT * FROM polls WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }

    pub async fn read_not_published(pool: &SqlitePool) -> Result<Option<Poll>, sqlx::Error>{
        let sql = "SELECT * FROM polls WHERE published = FALSE order by id";
        query(sql)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Poll>, sqlx::Error>{
        let sql = "SELECT * FROM polls";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, poll: Poll) -> Result<Poll, sqlx::Error>{
        let sql = "UPDATE polls SET category_id = $2, question = $3
                    FROM polls WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(poll.id)
            .bind(poll.category_id)
            .bind(poll.question)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Poll, sqlx::Error>{
        let sql = "DELETE from polls WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

}


