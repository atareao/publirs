CREATE TABLE IF NOT EXISTS tips (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_id INTEGER,
    title STRING,
    text STRING,
    published BOOLEAN
);
