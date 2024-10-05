DROP TABLE IF EXISTS high_scores;
DROP TABLE IF EXISTS games;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR(255) NOT NULL CHECK (first_name != ''),
  last_name VARCHAR(255) NOT NULL CHECK (last_name != ''),
  username VARCHAR(255) NOT NULL CHECK (image != ''),
  password_hash VARCHAR NOT NULL
);

CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL CHECK (title != ''),
  description TEXT NOT NULL CHECK (description != ''),
  slug VARCHAR(255) NOT NULL CHECK (slug != ''),
);

CREATE TABLE high_scores (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id),
  game_id INTEGER REFERENCES games(id),
  score INTEGER NOT NULL,
  lower_score_is_better BOOLEAN NOT NULL
);
