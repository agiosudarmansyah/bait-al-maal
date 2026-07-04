CREATE TABLE IF NOT EXISTS account ( 
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    icon_key TEXT NOT NULL,
    kind TEXT NOT NULL,
    provider TEXT NOT NULL,
    balance REAL NOT NULL
);