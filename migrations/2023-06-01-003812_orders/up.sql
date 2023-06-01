
CREATE TABLE customer_info (
  id serial PRIMARY KEY NOT NULL,
  phone text,
  name text NOT NULL
);

CREATE TABLE orders (
  id serial NOT NULL PRIMARY KEY,
  customer integer REFERENCES customer_info (id),
  components text NOT NULL,
  price float8 NOT NULL,
  status order_status NOT NULL DEFAULT 'Processing'
);