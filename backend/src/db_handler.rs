use sqlx::{postgres::PgRow, PgPool};
use std::error::Error;

pub struct DBHandler {
    db_connection: PgPool
}

impl DBHandler {
    pub async fn new() -> Result<DBHandler, Box<dyn Error>> {
        // initialize database
        let db = std::env::var("DB_URL").unwrap().to_string();
        let db_pool = PgPool::connect(db.as_str()).await?;

        sqlx::migrate!("./migrations").run(&db_pool).await?;

        Ok(DBHandler {
            db_connection: db_pool
        })

    }

    pub async fn get_all_notes(self) -> Result<PgRow, Box<dyn Error>>{
        let query = "SELECT * FROM notes";
        let sent_query = sqlx::query(query);
        let row = sent_query.fetch_one(&self.db_connection).await?;

        Ok(row)
    }

}
