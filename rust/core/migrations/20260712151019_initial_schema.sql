CREATE TABLE account ( 
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    icon_key TEXT NOT NULL,
    kind TEXT NOT NULL,
    provider TEXT,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL
);