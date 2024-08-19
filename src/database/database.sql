DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS games;


CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR(255) NOT NULL CHECK (first_name != ''),
  last_name VARCHAR(255) NOT NULL CHECK (last_name != ''),
  username VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL
);

CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL CHECK (title != ''),
  description TEXT NOT NULL CHECK (description != ''),
  image_url VARCHAR(255) NOT NULL CHECK (image_url != ''),
);

CREATE TABLE high_scores (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id),
  game_id INTEGER REFERENCES games(id),
  score INTEGER NOT NULL
);