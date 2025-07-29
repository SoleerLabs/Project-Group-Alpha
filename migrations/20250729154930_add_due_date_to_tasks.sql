-- Add migration script here
ALTER TABLE tasks ADD COLUMN due_date TIMESTAMPTZ;