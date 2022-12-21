CREATE TABLE snyk_events (
  id SERIAL PRIMARY KEY,
  ts TIMESTAMP NOT NULL,
  org JSONB NOT NULL,
  project JSONB NOT NULL,
  new_issues JSONB[] NOT NULL,
  removed_issues JSONB[] NOT NULL
)
