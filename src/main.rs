mod db;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let db_url = env!("DATABASE_URL");
    let pool = db::setup_db_connection(db_url).await.unwrap();

    let langs = db::get_all_languages(&pool).await.unwrap();
    for lang in &langs {
        println!("{lang:?}");
    }
}
