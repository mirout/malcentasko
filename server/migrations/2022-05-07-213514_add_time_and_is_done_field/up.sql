-- Your SQL goes here
ALTER TABLE tasks ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE tasks ADD COLUMN done_at TIMESTAMPTZ;
ALTER TABLE tasks ADD COLUMN is_done BOOLEAN NOT NULL DEFAULT false;