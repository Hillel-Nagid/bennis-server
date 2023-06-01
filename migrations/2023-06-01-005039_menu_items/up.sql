-- Your SQL goes here
CREATE TABLE menu_items(
  id serial PRIMARY KEY NOT NULL,
  name text NOT NULL,
  additions text,
  price float8 NOT NULL,
  image_url text
);