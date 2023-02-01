CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    address VARCHAR NOT NULL,
    user_name VARCHAR(30) NOT NULL 
);

CREATE TABLE trades (
  id serial,
  title VARCHAR NOT NULL,
  content text NOT NULL,
  created_by int NOT NULL,
  accepted_order_id INT NOT NULL,
  deposited_amount INT NOT NULL,
  buyer_address VARCHAR NOT NULL,
  seller_address VARCHAR NOT NULL,
  created_on timestamptz,
  PRIMARY KEY (id),
  FOREIGN KEY (created_by) REFERENCES users(id)
);