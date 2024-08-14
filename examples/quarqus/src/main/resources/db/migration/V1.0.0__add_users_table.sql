CREATE SEQUENCE user_id_seq;

CREATE TABLE IF NOT EXISTS users (
    id INTEGER NOT NULL DEFAULT nextval('user_id_seq'),
    email VARCHAR,
    password VARCHAR,
    role VARCHAR,
    PRIMARY KEY (id)
);
