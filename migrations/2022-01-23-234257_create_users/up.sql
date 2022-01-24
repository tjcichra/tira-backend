CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email_address TEXT,
    first_name TEXT,
    last_name TEXT
)