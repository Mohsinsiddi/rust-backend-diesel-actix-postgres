CREATE TABLE IF NOT EXISTS users
(
    id SERIAL PRIMARY KEY NOT NULL,
    address VARCHAR NOT NULL,
    user_name VARCHAR(30) NOT NULL 
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

CREATE TABLE IF NOT EXISTS trades
(
    id SERIAL PRIMARY KEY NOT NULL,
    users_id INT NOT NULL,
    accepted_order_id INT NOT NULL,
    buyer_address VARCHAR NOT NULL,
    seller_address VARCHAR NOT NULL,
    deposited_amount INT NOT NULL,

    FOREIGN KEY (users_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS orders 
(
    id SERIAL PRIMARY KEY NOT NULL,
    trade_id INT NOT NULL,
    collection_id INT NOT NULL,
    users_id INT NOT NULL,
    trade_amount INT NOT NULL,
    rarity VARCHAR(255) NOT NULL,
    collection_root VARCHAR(255) NOT NULL,
   
    FOREIGN KEY (trade_id) REFERENCES trades(id),
    FOREIGN KEY (collection_id) REFERENCES collections(id),
    FOREIGN KEY (users_id) REFERENCES users(id)
);

