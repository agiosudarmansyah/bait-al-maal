CREATE TABLE IF NOT EXISTS transaction (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    amount REAL NOT NULL,
    category TEXT NOT NULL
);