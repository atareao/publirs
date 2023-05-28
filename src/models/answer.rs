use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer{
    id: i64,
    poll_id: i64,
    text: String,
    #[serde(default = "get_default_isok")]
    isok: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAnswer{
    poll_id: i64,
    text: String,
    #[serde(default = "get_default_isok")]
    isok: bool
}

fn get_default_isok() -> bool{
    false
}

impl Answer{
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            poll_id: row.get("poll_id"),
            text: row.get("text"),
            isok: row.get("isok")
        }
    }

    pub fn get_id(&self) -> i64{
        self.id
    }

    pub fn get_poll_id(&self) -> i64{
        self.poll_id
    }

    pub fn get_text(&self) -> &str{
        &self.text
    }

    pub fn get_isok(&self) -> bool{
        self.isok
    }

    pub async fn create(pool: &SqlitePool, new_poll: NewAnswer)
            -> Result<Answer, sqlx::Error>{
        tracing::info!("Data: {:?}", new_poll);
        let sql = "INSERT INTO answers (poll_id, text, isok
                   VALUES ($1, $2, $3) RETURNING *;";
        query(sql)
            .bind(new_poll.poll_id)
            .bind(new_poll.text)
            .bind(new_poll.isok)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Option<Answer>, sqlx::Error>{
        let sql = "SELECT * FROM answers WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Answer>, sqlx::Error>{
        let sql = "SELECT * FROM answers";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn read_for_poll(pool: &SqlitePool, poll_id: i64) -> Result<Vec<Answer>, sqlx::Error>{
        let sql = "SELECT * FROM answers WHERE poll_id = $1 ORDER BY id";
        query(sql)
            .bind(poll_id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn get_correct_answer_for_poll(pool: &SqlitePool, poll_id: i64) -> Result<Option<i64>, sqlx::Error>{
        let sql = "SELECT * FROM answers WHERE poll_id = $1 ORDER BY id";
        query(sql)
            .bind(poll_id)
            .map(|result: SqliteRow| -> i64 {result.get(0)})
            .fetch_optional(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, answer: Answer) -> Result<Answer, sqlx::Error>{
        let sql = "UPDATE answers SET poll_id = $2, text = $3, isok = $4
                    FROM answers WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(answer.id)
            .bind(answer.poll_id)
            .bind(answer.text)
            .bind(answer.isok)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Answer, sqlx::Error>{
        let sql = "DELETE from answers WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}
