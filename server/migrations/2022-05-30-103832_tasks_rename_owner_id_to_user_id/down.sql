-- This file should undo anything in `up.sql`
ALTER TABLE tasks
RENAME user_id TO owner_id;
