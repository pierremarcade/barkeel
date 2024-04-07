CREATE TABLE sessions (
  session_token VARCHAR PRIMARY KEY,
  user_id INTEGER REFERENCES users(id)
);