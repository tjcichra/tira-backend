CREATE TABLE tickets (
    id BIGSERIAL PRIMARY KEY,
    category_id BIGINT REFERENCES categories (id),
    subject TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    priority TEXT NOT NULL,
    created TIMESTAMP NOT NULL,
    reporter_id BIGINT REFERENCES users (id) NOT NULL
);