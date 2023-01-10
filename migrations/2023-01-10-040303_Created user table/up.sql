-- Your SQL goes here
CREATE TABLE users (
    uuid serial NOT NULL,
    name character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    removed boolean NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (uuid),
    CONSTRAINT users_email_unique UNIQUE (email)
);