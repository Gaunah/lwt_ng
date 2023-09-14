CREATE TABLE languages (
    LanguageID INTEGER PRIMARY KEY,
    Name TEXT NOT NULL UNIQUE
);

CREATE TABLE texts (
    TextID INTEGER PRIMARY KEY,
    LanguageID INTEGER REFERENCES languages(LanguageID),
    Title TEXT NOT NULL,
    Content TEXT NOT NULL,
    SourceURL TEXT,
    AudioURL TEXT
);

CREATE TABLE tags (
    TagID INTEGER PRIMARY KEY,
    Name TEXT NOT NULL UNIQUE
);

CREATE TABLE text_tags (
    TextID INTEGER REFERENCES texts(TextID),
    TagID INTEGER REFERENCES tags(TagID),
    PRIMARY KEY (TextID, TagID)
);

CREATE TABLE words (
    WordID INTEGER PRIMARY KEY,
    Word TEXT NOT NULL,
    Translation TEXT NOT NULL,
    LanguageID INTEGER REFERENCES languages(LanguageID),
    LearningGrade INTEGER NOT NULL CHECK (LearningGrade >= 0 AND LearningGrade <= 4),
    Notes TEXT
);

CREATE TABLE example_sentences (
    SentenceID INTEGER PRIMARY KEY,
    WordID INTEGER REFERENCES words(WordID),
    Content TEXT NOT NULL
);