CREATE TABLE short_links (
  "id" bigserial NOT NULL PRIMARY KEY,
  "url" varchar,
  "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);