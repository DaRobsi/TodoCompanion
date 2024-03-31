use sqlx::{postgres, Connection, PgConnection};
use std::error::Error;

pub struct DBHandler {
    db_connection: PgConnection
}

impl DBHandler {
    pub async fn db_init(&self) -> Result<DBHandler, Box<dyn Error>> {
            // initialize database
            let db = std::env::var("DB_URL").unwrap().to_string();
            let mut db_conn = postgres::PgConnection::connect(db.as_str()).await?;
            
            Ok(DBHandler { db_connection: db_conn })
        }

}