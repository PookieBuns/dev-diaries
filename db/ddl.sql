CREATE TABLE
  "user" (
    user_id SERIAL PRIMARY KEY,
    user_name VARCHAR(255) NOT NULL UNIQUE,
    salt BYTEA NOT NULL,
    password_hash BYTEA NOT NULL
  );
