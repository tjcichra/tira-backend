CREATE TABLE sessions (
    uuid TEXT PRIMARY KEY,
    user_id BIGINT REFERENCES users (id) NOT NULL,
    created TIMESTAMP NOT NULL,
    expiration TIMESTAMP
);