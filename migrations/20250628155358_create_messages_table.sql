-- Add migration script here
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL,
    content TEXT NOT NULL
);
