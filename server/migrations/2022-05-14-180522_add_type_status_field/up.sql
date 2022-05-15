-- Your SQL goes here

CREATE TABLE IF NOT EXISTS task_status_for_user (
    id uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT table_task_status_for_user_pk PRIMARY KEY,
    user_id uuid references users NOT NULL,
    status_name VARCHAR(64)
);

CREATE UNIQUE INDEX IF NOT EXISTS status_for_user_uindex ON task_status_for_user (user_id, status_name);

CREATE TABLE IF NOT EXISTS task_types_for_user (
    id uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT table_task_tupes_for_user_pk PRIMARY KEY,
    user_id uuid references users NOT NULL,
    type_name VARCHAR(128)
);

CREATE UNIQUE INDEX IF NOT EXISTS type_for_user_uindex ON task_types_for_user (user_id, type_name);

ALTER TABLE tasks DROP COLUMN done_at;
ALTER TABLE tasks DROP COLUMN is_done;

ALTER TABLE tasks ADD COLUMN status_name uuid references task_status_for_user;
ALTER TABLE tasks ADD COLUMN type_name uuid references task_types_for_user;
