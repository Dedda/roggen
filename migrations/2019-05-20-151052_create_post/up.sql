CREATE TABLE post (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  published TIMESTAMP DEFAULT NULL,
  blog VARCHAR NOT NULL,
  created TIMESTAMP NOT NULL
)