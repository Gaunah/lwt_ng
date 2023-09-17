use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, FromRow, Pool, Sqlite};

#[derive(FromRow, Debug)]
pub struct Text {
    language_id: i64,
    title: String,
    content: String,
    source_url: String,
    audio_url: String,
}

impl Text {
    pub fn new(language: &Language, title: &str, content: &str) -> Self {
        Text {
            language_id: language.language_id,
            title: String::from(title),
            content: String::from(content),
            source_url: String::new(),
            audio_url: String::new(),
        }
    }
}

#[derive(FromRow, Debug)]
pub struct Language {
    language_id: i64,
    name: String,
}

impl Language {
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub async fn add_language(
    name: &str,
    pool: &Pool<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!("INSERT INTO Languages (name) VALUES ($1)", name)
        .execute(pool)
        .await
}

pub async fn get_all_languages(pool: &Pool<Sqlite>) -> Result<Vec<Language>, sqlx::Error> {
    sqlx::query_as!(Language, "SELECT * FROM Languages")
        .fetch_all(pool)
        .await
}

pub async fn add_text(
    text: &Text,
    pool: &Pool<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO texts (language_id, title, content, source_url, audio_url)
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
}

pub async fn setup_db_connection(db_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating new database {}", db_url);
        Sqlite::create_database(db_url).await?;
    }

    let pool = SqlitePool::connect(db_url).await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn insert_and_read_language(pool: Pool<Sqlite>) {
        assert!(add_language("test", &pool).await.is_ok());
        let langs = get_all_languages(&pool).await.unwrap();
        assert_eq!(langs.first().unwrap().name(), "test");
    }
}
