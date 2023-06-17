use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use super::error::CustomError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tip{
    id: i64,
    category_id: i64,
    text: String,
    #[serde(default = "get_default_published")]
    published: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTip{
    category_id: i64,
    text: String,
    #[serde(default = "get_default_published")]
    published: bool
}

fn get_default_published() -> bool{
    false
}

impl Tip{
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            category_id: row.get("category_id"),
            text: row.get("text"),
            published: row.get("published")
        }
    }

    pub fn get_id(&self) -> i64{
        self.id
    }

    pub fn get_category_id(&self) -> i64{
        self.category_id
    }

    pub fn get_text(&self) -> &str{
        &self.text
    }

    pub fn get_published(&self) -> bool{
        self.published
    }

    pub async fn create(pool: &SqlitePool, new_tip: NewTip)
            -> Result<Tip, CustomError>{
        tracing::info!("Data: {:?}", new_tip);
        let sql = "INSERT INTO tips (category_id, text, published
                   VALUES ($1, $2, $3) RETURNING *;";
        query(sql)
            .bind(new_tip.category_id)
            .bind(new_tip.text)
            .bind(new_tip.published)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                CustomError::ServerError(e.to_string())
            })
    }
    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Option<Tip>, CustomError>{
        let sql = "SELECT * FROM tips WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                CustomError::ServerError(e.to_string())
            })
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Tip>, CustomError>{
        let sql = "SELECT * FROM tips";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                CustomError::ServerError(e.to_string())
            })
    }

    pub async fn read_not_published(pool: &SqlitePool) -> Result<Option<Tip>, CustomError>{
        let sql = "SELECT * FROM tips WHERE published = FALSE ORDER BY id LIMIT 1";
        query(sql)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                CustomError::ServerError(e.to_string())
            })
    }

    pub async fn update(pool: &SqlitePool, tip: Tip) -> Result<Tip, CustomError>{
        let sql = "UPDATE tips SET category_id = $2, text = $3, published = $4
                    FROM tips WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(tip.id)
            .bind(tip.category_id)
            .bind(tip.text)
            .bind(tip.published)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                CustomError::ServerError(e.to_string())
            })
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Tip, CustomError>{
        let sql = "DELETE from tips WHERE id = $1 RETURNING * ;";
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


