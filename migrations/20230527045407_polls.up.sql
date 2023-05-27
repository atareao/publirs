CREATE TABLE IF NOT EXISTS polls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_id INTEGER,
    question STRING,
    published BOOLEAN DEFAULT false
);
