use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use super::{
    answer::{Answer, NewBasicAnswer},
    error::CustomError
};

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
    pub category: String,
    pub question: String,
    pub answers: Vec<NewBasicAnswer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollWithAnswers{
    id: i64,
    category_id: i64,
    question: String,
    published: bool,
    pub answers: Vec<Answer>,
}

fn get_default_published() -> bool{
    false
}

impl NewPoll{
    pub fn new(category_id: i64, question: String) -> Self{
        Self{
            category_id,
            question,
        }
    }
}
impl PollWithAnswers{
    pub fn new(id: i64, category_id: i64, question: String, published: bool, answers: Vec<Answer>) -> Self{
        Self{
            id,
            category_id,
            question,
            published,
            answers,
        }
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

    pub fn set_published(&mut self, published: bool){
        self.published = published;
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
    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Option<Poll>, CustomError>{
        let sql = "SELECT * FROM polls WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
            .map_err(|_| {
                CustomError::NotFound
            })
    }

    pub async fn read_not_published(pool: &SqlitePool) -> Result<Option<Poll>, CustomError>{
        let sql = "SELECT * FROM polls WHERE published = FALSE ORDER BY id LIMIT 1";
        query(sql)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                CustomError::ServerError(e.to_string())
            })
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Poll>, CustomError>{
        let sql = "SELECT * FROM polls";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|_| {
                CustomError::NotFound
            })
    }

    pub async fn update(pool: &SqlitePool, poll: Poll) -> Result<Poll, CustomError>{
        let sql = "UPDATE polls SET category_id = $2, question = $3,
                    published = $4 WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(poll.id)
            .bind(poll.category_id)
            .bind(poll.question)
            .bind(poll.published)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Error: {}", e);
                CustomError::ServerError(e.to_string())
            })
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Poll, CustomError>{
        let sql = "DELETE from polls WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                CustomError::ServerError(e.to_string())
            })
    }
}
