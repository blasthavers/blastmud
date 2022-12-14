-- Note database created is ephemeral and use for migra to diff only.
-- Never put data in it, or it will be lost.
DROP DATABASE IF EXISTS blast_schemaonly;
CREATE DATABASE blast_schemaonly;

\c blast_schemaonly

CREATE TABLE listeners (
  listener UUID NOT NULL PRIMARY KEY,
  last_seen TIMESTAMP WITH TIME ZONE
);

CREATE TABLE sessions (
  session UUID NOT NULL PRIMARY KEY,
  listener UUID NOT NULL,
  details JSONB NOT NULL
);
CREATE INDEX session_by_listener ON sessions(listener);

CREATE TABLE items (
  item_id BIGINT NOT NULL PRIMARY KEY,
  item_code TEXT NOT NULL,
  item_type TEXT NOT NULL,
  location BIGINT REFERENCES items(item_id),
  details JSONB NOT NULL,
  UNIQUE (item_code, item_type)
);
CREATE INDEX item_index ON items (item_code, item_type);
CREATE INDEX item_by_loc ON items (location);
  
CREATE TABLE users (
  username TEXT NOT NULL PRIMARY KEY,
  current_session UUID REFERENCES sessions(session),
  current_listener UUID REFERENCES listeners(listener),
  details JSONB NOT NULL
);
CREATE INDEX user_by_listener ON users(current_listener);

CREATE UNLOGGED TABLE sendqueue (
  item BIGINT NOT NULL PRIMARY KEY,
  session UUID NOT NULL REFERENCES sessions(session),
  listener UUID REFERENCES listeners(listener),
  message TEXT NOT NULL
);

CREATE TABLE tasks (
  task_code TEXT NOT NULL,
  task_type TEXT NOT NULL,
  next_scheduled TIMESTAMP WITH TIME ZONE NOT NULL,
  details JSONB NOT NULL,
  PRIMARY KEY (task_code, task_type)
);
CREATE INDEX task_by_next_scheduled ON tasks(next_scheduled);
