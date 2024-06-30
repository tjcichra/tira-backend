CREATE TABLE comments (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT REFERENCES tickets (id) NOT NULL,
    commenter_id BIGINT REFERENCES users (id) NOT NULL,
    content TEXT NOT NULL,
    commented TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);