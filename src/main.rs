use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, Pool, Sqlite};
use std::process;

const DB_URL: &str = "sqlite://lwt_ng.db";

#[tokio::main]
async fn main() {
    let pool = setup_db().await.unwrap();
}

async fn setup_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database exists");
    }

    let pool = SqlitePool::connect(DB_URL).await?;
    if let Err(e) = sqlx::migrate!().run(&pool).await {
        eprintln!("error: {e}");
        process::exit(1);
    }

    Ok(pool)
}
