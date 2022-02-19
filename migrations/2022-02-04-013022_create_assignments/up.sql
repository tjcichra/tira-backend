CREATE TABLE assignments (
    ticket_id BIGINT REFERENCES tickets (id) NOT NULL,
    user_id BIGINT REFERENCES users (id) NOT NULL,
    assigned TIMESTAMP NOT NULL,
    PRIMARY KEY (ticket_id, user_id)
);