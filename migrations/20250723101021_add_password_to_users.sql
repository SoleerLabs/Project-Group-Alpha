-- Add migration script here
ALTER TABLE users
ADD COLUMN password VARCHAR(255) NOT NULL;