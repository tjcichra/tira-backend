CREATE TABLE sessions (
    uuid TEXT PRIMARY KEY,
    user_id INTEGER REFERENCES users (id) NOT NULL,
    created TIMESTAMP NOT NULL,
    expiration TIMESTAMP NOT NULL
);