CREATE TABLE IF NOT EXISTS orders 
(
    id SERIAL UNIQUE PRIMARY KEY NOT NULL,
    trade_id INT NOT NULL,
    collection_id INT NOT NULL,
    user_id INT NOT NULL,
    trade_amount INT NOT NULL,
    rarity VARCHAR(255) NOT NULL,
    collection_root VARCHAR(255) NOT NULL,
   
    FOREIGN KEY (trade_id) REFERENCES trades(id),
    FOREIGN KEY (collection_id) REFERENCES collections(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);