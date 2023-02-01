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

CREATE TABLE IF NOT EXISTS collections
(
    id SERIAL PRIMARY KEY NOT NULL,
    collection_name VARCHAR(30) NOT NULL,
    ceiling_price INT NOT NULL,
    active_trades INT NOT NULL,
    total_trades INT NOT NULL,
    volume INT NOT NULL,
    supply INT NOT NULL
);