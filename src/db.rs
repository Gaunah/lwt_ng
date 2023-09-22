use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, FromRow, Pool, Sqlite};

pub struct Language {
    language_id: i64,
    name: String,
}

impl Language {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Word {
    word_id: i64,
    word: String,
    pub translation: String,
    language_id: i64,
    pub learning_grade: i64,
    pub notes: Option<String>,
}

impl Word {
    pub fn new(word: &str, language: Language) -> Self {
        Word {
            word_id: -1,
            word: String::from(word),
            translation: String::new(),
            language_id: language.language_id,
            learning_grade: 0,
            notes: None,
        }
    }

    pub fn word(&self) -> &str {
        &self.word
    }
}

pub async fn add_word(
    word: &Word,
    pool: &Pool<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "
        INSERT INTO words (word, translation, language_id, learning_grade, notes)
        VALUES ($1,$2, $3, $4, $5)
        ",
        word.word,
        word.translation,
        word.language_id,
        word.learning_grade,
        word.notes
    )
    .execute(pool)
    .await
}

pub async fn get_all_words(pool: &Pool<Sqlite>) -> Result<Vec<Word>, sqlx::Error> {
    sqlx::query_as!(Word, "SELECT * FROM words")
        .fetch_all(pool)
        .await
}

pub async fn add_language(
    name: &str,
    pool: &Pool<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!("INSERT INTO languages (name) VALUES ($1)", name)
        .execute(pool)
        .await
}

pub async fn get_all_languages(pool: &Pool<Sqlite>) -> Result<Vec<Language>, sqlx::Error> {
    sqlx::query_as!(Language, "SELECT * FROM languages")
        .fetch_all(pool)
        .await
}

pub async fn add_text(
    text: &Text,
    pool: &Pool<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "
        INSERT INTO texts (language_id, title, content, source_url, audio_url)
        VALUES ($1, $2, $3, $4, $5)
        ",
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
