CREATE TABLE languages (
    language_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE texts (
    text_id INTEGER PRIMARY KEY,
    language_id INTEGER NOT NULL REFERENCES languages(language_id),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    source_url TEXT,
    audio_url TEXT
);

CREATE TABLE tags (
    tag_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE text_tags (
    text_id INTEGER NOT NULL REFERENCES texts(text_id),
    tag_id INTEGER NOT NULL REFERENCES tags(tag_id),
    PRIMARY KEY (text_id, tag_id)
);

CREATE TABLE words (
    word_id INTEGER PRIMARY KEY,
    word TEXT NOT NULL UNIQUE,
    translation TEXT NOT NULL,
    language_id INTEGER NOT NULL REFERENCES languages(language_id),
    learning_grade INTEGER NOT NULL CHECK (learning_grade >= 0 AND learning_grade <= 4),
    notes TEXT
);

CREATE TABLE example_sentences (
    sentence_id INTEGER PRIMARY KEY,
    word_id INTEGER NOT NULL REFERENCES words(word_id),
    content TEXT NOT NULL
);