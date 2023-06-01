-- Your SQL goes here
CREATE TABLE additions(
  id serial PRIMARY KEY NOT NULL,
  name text NOT NULL,
  price float8 NOT NULL,
  image_url text
);