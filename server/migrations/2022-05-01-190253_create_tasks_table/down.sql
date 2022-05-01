-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS tasks_id_uindex;
DROP INDEX IF EXISTS tasks_owner_id_index;
DROP INDEX IF EXISTS tasks_parent_id_index;

DROP TABLE IF EXISTS tasks;