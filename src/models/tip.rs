use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Option{
    id: i64,
    reto_id: i64,
    option: String,
    #[serde(default = "get_default_isok")]
    isok: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTip{
    reto_id: i64,
    option: String,
    #[serde(default = "get_default_isok")]
    isok: bool
}

fn get_default_isok() -> bool{
    false
}

impl Option{
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            reto_id: row.get("reto_id"),
            option: row.get("option"),
            isok: row.get("isok")
        }
    }

    pub fn get_id(&self) -> i64{
        self.id
    }

    pub fn get_reto_id(&self) -> i64{
        self.reto_id
    }

    pub fn get_option(&self) -> &str{
        &self.option
    }

    pub fn get_isok(&self) -> bool{
        self.isok
    }

    pub async fn create(pool: &SqlitePool, new_poll: NewTip)
            -> Result<Option, sqlx::Error>{
        tracing::info!("Data: {:?}", new_poll);
        let sql = "INSERT INTO options (reto_id, option, isok
                   VALUES ($1, $2, $3) RETURNING *;";
        query(sql)
            .bind(new_poll.reto_id)
            .bind(new_poll.option)
            .bind(new_poll.isok)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Option<Option>, sqlx::Error>{
        let sql = "SELECT * FROM options WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Option>, sqlx::Error>{
        let sql = "SELECT * FROM options";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, option: Option) -> Result<Option, sqlx::Error>{
        let sql = "UPDATE options SET reto_id = $2, option = $3, isok = $4
                    FROM options WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(option.id)
            .bind(option.reto_id)
            .bind(option.option)
            .bind(option.isok)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Option, sqlx::Error>{
        let sql = "DELETE from options WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

}


