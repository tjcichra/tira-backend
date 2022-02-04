CREATE TABLE assignments (
    ticket_id INTEGER REFERENCES tickets (id) NOT NULL,
    user_id INTEGER REFERENCES users (id) NOT NULL,
    assigned TIMESTAMP NOT NULL,
    PRIMARY KEY (ticket_id, user_id)
);