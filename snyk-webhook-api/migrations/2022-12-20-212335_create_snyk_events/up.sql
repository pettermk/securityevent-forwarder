CREATE TABLE snyk_events (
  id SERIAL PRIMARY KEY,
  ts TIMESTAMP NOT NULL,
  org JSONB DEFAULT '{}'::jsonb NOT NULL,
  project JSONB DEFAULT '{}'::jsonb NOT NULL,
  new_issues JSONB[] DEFAULT '{}' NOT NULL,
  removed_issues JSONB[] DEFAULT '{}' NOT NULL
)
