CREATE TABLE heading(
  id SERIAL PRIMARY KEY,
  post INTEGER NOT NULL REFERENCES post(id),
  section_index INTEGER NOT NULL,
  heading_text TEXT NOT NULL DEFAULT '',
  heading_size INTEGER NOT NULL DEFAULT 1
)