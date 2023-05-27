CREATE TABLE IF NOT EXISTS tips (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    norder INTEGER,
    category_id INTEGER,
    tip STRING,
    published DATETIME
);
