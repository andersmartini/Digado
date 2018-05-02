CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  message TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 't'
)