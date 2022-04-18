CREATE TABLE tickets (
    id BIGSERIAL PRIMARY KEY,
    subject TEXT NOT NULL,
    description TEXT,
    category_id BIGINT REFERENCES categories (id),
    priority TEXT NOT NULL,
    status TEXT NOT NULL,
    created TIMESTAMP NOT NULL,
    reporter_id BIGINT REFERENCES users (id) NOT NULL
);