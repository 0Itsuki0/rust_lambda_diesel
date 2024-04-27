-- Your SQL goes here

CREATE TABLE events (
  id VARCHAR PRIMARY KEY,
  name VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL REFERENCES users(id)
)
