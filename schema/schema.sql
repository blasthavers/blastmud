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
  item_id BIGSERIAL NOT NULL PRIMARY KEY,
  details JSONB NOT NULL
);
CREATE UNIQUE INDEX item_index ON items ((details->>'item_code'), (details->>'item_type'));
CREATE INDEX item_by_loc ON items ((details->>'location'));
CREATE INDEX item_by_static ON items ((cast(details->>'is_static' as boolean)));
  
CREATE TABLE users (
  username TEXT NOT NULL PRIMARY KEY,
  current_session UUID REFERENCES sessions(session),
  current_listener UUID REFERENCES listeners(listener),
  details JSONB NOT NULL
);
CREATE INDEX user_by_listener ON users(current_listener);
CREATE INDEX user_by_session ON users(current_session);

CREATE UNLOGGED TABLE sendqueue (
  item BIGSERIAL NOT NULL PRIMARY KEY,
  session UUID NOT NULL REFERENCES sessions(session),
  listener UUID REFERENCES listeners(listener),
  message TEXT /* Nullable, null means disconnect */
);

CREATE TABLE tasks (
  task_id BIGSERIAL NOT NULL PRIMARY KEY,
  details JSONB NOT NULL
);
CREATE UNIQUE INDEX tasks_by_code_type ON tasks((details->>'task_code'), (details->>'task_type'));
CREATE INDEX tasks_by_static ON tasks((cast(details->>'is_static' as boolean)));
CREATE INDEX tasks_by_scheduled ON tasks((details->>'next_scheduled'));
