use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category{
    id: i64,
    name: String,
    chat_id: String,
    thread_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewCategory{
    name: String,
    chat_id: String,
    thread_id: i64,
}

impl Category{
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            name: row.get("name"),
            chat_id: row.get("chat_id"),
            thread_id: row.get("thread_id")
        }
    }

    pub fn get_id(&self) -> i64{
        self.id
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_chat_id(&self) -> &str{
        &self.chat_id
    }

    pub fn get_thread_id(&self) -> i64{
        self.thread_id
    }

    pub async fn create(pool: &SqlitePool, new_category: NewCategory)
            -> Result<Category, sqlx::Error>{
        tracing::info!("Data: {:?}", new_category);
        let sql = "INSERT INTO categories (name, chat_id, thread_id) 
                   VALUES ($1, $2, $3) RETURNING *;";
        query(sql)
            .bind(new_category.name)
            .bind(new_category.chat_id)
            .bind(new_category.thread_id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Option<Category>, sqlx::Error>{
        let sql = "SELECT * FROM categories WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Category>, sqlx::Error>{
        let sql = "SELECT * FROM categories";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, category: Category) -> Result<Category, sqlx::Error>{
        let sql = "UPDATE categories SET name = $2, chat_id = $3, thread_id = $4
                    FROM categories WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(category.id)
            .bind(category.name)
            .bind(category.chat_id)
            .bind(category.thread_id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Category, sqlx::Error>{
        let sql = "DELETE from categories WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

}


