-- Add migration script here
CREATE TABLE messages{
id SERIAL PRIMARY KEY,
username VARCHAR(255) NOT NULL,
content TEXT NOT NULL,
token VARCHAR(255) NOT NULL
}
