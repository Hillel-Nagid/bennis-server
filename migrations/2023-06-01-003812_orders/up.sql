
CREATE TABLE customer_info (
  id serial PRIMARY KEY NOT NULL,
  name text NOT NULL,
  phone text
);

CREATE TABLE orders (
  id serial NOT NULL PRIMARY KEY,
  customer_id integer NOT NULL REFERENCES customer_info (id),
  customer_name text NOT NULL,
  components text NOT NULL,
  price float8 NOT NULL,
  status integer DEFAULT 0
);