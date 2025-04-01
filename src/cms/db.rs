use sqlx::{SqlitePool, migrate::MigrateDatabase};
use std::string::String;

pub async fn init_db(db_url: &str) -> SqlitePool {
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap() {
        sqlx::Sqlite::create_database(db_url).await.unwrap();
    }
    let pool = SqlitePool::connect(db_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[derive(sqlx::FromRow)]
pub struct Transaction {
    pub id: String,
    pub charger_id: String,
    pub energy: f64,
    pub cost: f64,
    pub timestamp: String,
}



