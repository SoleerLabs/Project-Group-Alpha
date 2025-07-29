-- Add migration script here
CREATE TYPE task_status AS ENUM ('pending', 'in_progress', 'completed');

ALTER TABLE tasks
ALTER COLUMN status DROP DEFAULT;

ALTER TABLE tasks
ALTER COLUMN status TYPE task_status
USING status::task_status,
ALTER COLUMN status SET DEFAULT 'pending',
ALTER COLUMN status SET NOT NULL;
