CREATE TABLE IF NOT EXISTS encounters (
    id SERIAL PRIMARY KEY,
    name TEXT,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
)