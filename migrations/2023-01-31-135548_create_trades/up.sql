CREATE TABLE IF NOT EXISTS trades
(
    id SERIAL UNIQUE PRIMARY KEY NOT NULL,
    collection_id INT NOT NULL,
    user_id INT NOT NULL UNIQUE,
    accepted boolean NOT NULL,
    accepted_order_id INT,
    buyer_address BYTEA NOT NULL,
    seller_address BYTEA,
    deposited_amount INT NOT NULL,

    FOREIGN KEY (collection_id) REFERENCES collections(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);