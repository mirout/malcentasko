-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_role_t as enum ('admin', 'user');

CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT table_name_pk PRIMARY KEY,
    username VARCHAR(128) NOT NULL,
    user_role user_role_t NOT NULL DEFAULT 'user',
    secret TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS users_id_uindex ON users (id);
CREATE UNIQUE INDEX IF NOT EXISTS users_username_uindex ON users (username);