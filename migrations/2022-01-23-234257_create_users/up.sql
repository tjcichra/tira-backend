CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email_address TEXT,
    first_name TEXT,
    last_name TEXT
)