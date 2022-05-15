-- This file should undo anything in `up.sql`

ALTER TABLE tasks DROP COLUMN type_name;
ALTER TABLE tasks DROP COLUMN status_name;

DROP INDEX IF EXISTS status_for_user_uindex;
DROP INDEX IF EXISTS type_for_user_uindex;

DROP TABLE IF EXISTS task_status_for_user;
DROP TABLE IF EXISTS task_type_for_user;

ALTER TABLE tasks ADD COLUMN done_at TIMESTAMPTZ;
ALTER TABLE tasks ADD COLUMN is_done BOOLEAN NOT NULL DEFAULT false;
