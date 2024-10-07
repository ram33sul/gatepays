use sea_orm::{Database, DatabaseConnection};

pub async fn init_db(database_url: &str) -> DatabaseConnection {
    Database::connect(database_url).await.unwrap()
}
