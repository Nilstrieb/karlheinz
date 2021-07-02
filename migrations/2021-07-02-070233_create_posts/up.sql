-- Your SQL goes here

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL DEFAULT ''
);