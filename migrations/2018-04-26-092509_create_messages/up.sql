CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  website VARCHAR NOT NULL,
  user_name VARCHAR NOT NULL,
  message TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 't'
)