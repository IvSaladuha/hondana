-- Your SQL goes here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    author VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    isbn VARCHAR DEFAULT ''
)
