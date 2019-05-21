CREATE TABLE text_section(
  id SERIAL PRIMARY KEY,
  post INTEGER REFERENCES post(id),
  section_index INTEGER NOT NULL,
  section_text TEXT NOT NULL DEFAULT ''
)