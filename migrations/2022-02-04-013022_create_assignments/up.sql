CREATE TABLE assignments (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT REFERENCES tickets (id) NOT NULL,
    assignee_id BIGINT REFERENCES users (id) NOT NULL,
    assigner_id BIGINT REFERENCES users (id) NOT NULL,
    assigned TIMESTAMP NOT NULL
);