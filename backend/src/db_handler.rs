use sqlx::{postgres::PgRow, PgPool};
use std::error::Error;
use dotenv::dotenv;

use crate::models::Note;

#[derive(Debug)]
pub struct DBHandler {
    db_connection: PgPool,
}

impl DBHandler {
    pub async fn new() -> Result<DBHandler, Box<dyn Error>> {
        dotenv().ok();
        // initialize database
        let db = std::env::var("DB_URL").unwrap().to_string();
        let db_pool = PgPool::connect(db.as_str()).await?;

        sqlx::migrate!("./migrations").run(&db_pool).await?;

        Ok(DBHandler {
            db_connection: db_pool,
        })
    }

    pub async fn get_all_notes(&self) -> Result<Vec<PgRow>, Box<dyn Error>> {
        let query = "SELECT * FROM notes";
        let sent_query = sqlx::query(query);
        let row = sent_query.fetch_all(&self.db_connection).await?;

        Ok(row)
    }

    pub async fn get_note(&self, id: &String) -> Result<PgRow, Box<dyn Error>> {
        let query = "SELECT * FROM notes WHERE id = $1";
        let sent_query = sqlx::query(query).bind(&id);
        let row = sent_query.fetch_one(&self.db_connection).await?;

        Ok(row)
    }

    //pub async fn create_note(&self, note: &Note) -> Result<>
}
