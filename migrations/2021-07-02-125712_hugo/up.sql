-- Your SQL goes here

CREATE TABLE person (
    id CHAR(4) NOT NULL PRIMARY KEY ,
    name VARCHAR(45) NOT NULL,
    age INT NOT NULL DEFAULT 0
);