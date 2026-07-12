CREATE TABLE account ( 
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    icon_key TEXT NOT NULL,
    kind TEXT NOT NULL,
    provider TEXT,
    amount BIGINT NOT NULL,
    currency TEXT NOT NULL
);