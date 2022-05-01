-- Your SQL goes here

CREATE TABLE IF NOT EXISTS tasks (
    id uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT table_tasks_pk PRIMARY KEY,
    owner_id uuid references users NOT NULL,
    parent_id uuid references tasks,
    title VARCHAR(128) NOT NULL,
    task_description TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS tasks_id_uindex ON tasks (id);
CREATE INDEX IF NOT EXISTS tasks_owner_id_index ON tasks (owner_id);
CREATE INDEX IF NOT EXISTS tasks_parent_id_index ON tasks (parent_id);
