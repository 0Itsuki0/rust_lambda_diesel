-- This file should undo anything in `up.sql`

ALTER TABLE users
DROP COLUMN language;
DROP TYPE Language

