use sqlx::{postgres, Connection, PgConnection};
use std::error::Error;
use log::info;

pub struct DBHandler {
    db_connection: PgConnection
}

impl DBHandler {
    pub async fn new() -> Result<DBHandler, Box<dyn Error>> {
        // initialize database
        let db = std::env::var("DB_URL").unwrap().to_string();
        let mut db_conn = postgres::PgConnection::connect(db.as_str()).await?;
        
        Ok(DBHandler { db_connection: db_conn })
    }

    pub async fn ping_database(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Checking Database...");
        let res = PgConnection::ping(&mut self.db_connection);
        
        Ok(())
    }
}