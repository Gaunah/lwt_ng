use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, FromRow, Pool, Sqlite};
use std::process;

#[tokio::main]
async fn main() {
    let pool = setup_db().await.unwrap();
    let text = Text::new(1, "Lorem ipsum",
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
            sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
            Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
            Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
            Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");

    add_language("test lang", &pool).await;
    add_text(&text, &pool).await;
}

#[derive(FromRow, Debug)]
struct Text {
    language_id: u32,
    title: String,
    content: String,
    source_url: String,
    audio_url: String,
}

impl Text {
    fn new(language_id: u32, title: &str, content: &str) -> Self {
        Text {
            language_id: language_id,
            title: String::from(title),
            content: String::from(content),
            source_url: String::new(),
            audio_url: String::new(),
        }
    }
}

async fn add_language(name: &str, pool: &Pool<Sqlite>) {
    if let Err(e) = sqlx::query!(
        r#"
        INSERT INTO Languages (Name)
        VALUES ($1)
        "#,
        name
    )
    .execute(pool)
    .await
    {
        eprintln!("error: {e}");
        process::exit(1);
    };
}

async fn add_text(text: &Text, pool: &Pool<Sqlite>) {
    if let Err(e) = sqlx::query!(
        r#"
        INSERT INTO texts (LanguageID, Title, Content, SourceURL, AudioURL)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        text.language_id,
        text.title,
        text.content,
        text.source_url,
        text.audio_url
    )
    .execute(pool)
    .await
    {
        eprintln!("error: {e}");
        process::exit(1);
    };
}

async fn setup_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    //TODO use dotenvy
    let db_url: &str = "sqlite://lwt_ng.db";

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => eprintln!("error: {}", error),
        }
    }

    let pool = SqlitePool::connect(db_url).await?;
    if let Err(e) = sqlx::migrate!().run(&pool).await {
        eprintln!("error: {e}");
        process::exit(1);
    }

    Ok(pool)
}
