CREATE TABLE tickets (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES categories (id),
    subject TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    priority TEXT NOT NULL,
    created TIMESTAMP NOT NULL,
    reporter_id INTEGER REFERENCES users (id) NOT NULL
);