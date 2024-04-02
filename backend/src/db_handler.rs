use sqlx::PgPool;
use std::error::Error;

pub struct DBHandler {
    db_connection: PgPool
}

impl DBHandler {
    pub async fn new() -> Result<DBHandler, Box<dyn Error>> {
        // initialize database
        let db = std::env::var("DB_URL").unwrap().to_string();
        let db_pool = PgPool::connect(db.as_str()).await?;

        Ok(DBHandler {
            db_connection: db_pool
        })
    }

    //* Healthcheck with ping wont work with pool
    /*pub async fn ping_database(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Checking Database...");
        let res = PgConnection::ping(&mut self.pg_connection).await?;

        Ok(res)
    } */
}
