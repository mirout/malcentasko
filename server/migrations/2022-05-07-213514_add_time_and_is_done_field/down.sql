-- This file should undo anything in `up.sql`'
ALTER TABLE tasks DROP COLUMN created_at;
ALTER TABLE tasks DROP COLUMN done_at;
ALTER TABLE tasks DROP COLUMN is_done;