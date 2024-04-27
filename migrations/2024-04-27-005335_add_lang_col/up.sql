-- Your SQL goes here
CREATE TYPE Language AS ENUM (
    'EN', 'JP'
);

ALTER TABLE users
ADD language Language NOT NULL DEFAULT 'EN';