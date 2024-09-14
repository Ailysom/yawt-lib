-- Your SQL goes here
CREATE TABLE task (
  id TEXT NOT NULL PRIMARY KEY,
  description TEXT NOT NULL,
  priority INTEGER NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 5,
  deadline TEXT NOT NULL,
  time_stamp TEXT NOT NULL
)
