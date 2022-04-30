-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS users_username_uindex;

DROP INDEX IF EXISTS users_id_uindex;

DROP TABLE IF EXISTS users;

DROP TYPE user_role_t;