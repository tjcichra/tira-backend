CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    ticket_id INTEGER REFERENCES tickets (id) NOT NULL,
    commenter_id INTEGER REFERENCES users (id) NOT NULL,
    content TEXT NOT NULL,
    commented TIMESTAMP NOT NULL
);