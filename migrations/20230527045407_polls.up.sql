CREATE TABLE IF NOT EXISTS polls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    norder INTEGER TYPE UNIQUE,
    category_id INTEGER,
    question STRING,
    published DATETIME
);
