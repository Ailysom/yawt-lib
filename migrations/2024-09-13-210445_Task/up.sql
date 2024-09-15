-- Your SQL goes here
CREATE TABLE task (
  id TEXT NOT NULL PRIMARY KEY,
  description TEXT NOT NULL,
  priority INTEGER NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 5,
  deadline DATETIME NOT NULL,
  time_stamp DATETIME NOT NULL
)
