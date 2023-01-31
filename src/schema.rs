// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Int4,
        collection_name -> Varchar,
        ceiling_price -> Int4,
        active_trades -> Int4,
        total_trades -> Int4,
        volume -> Int4,
        supply -> Int4,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        trade_id -> Int4,
        collection_id -> Int4,
        user_id -> Int4,
        trade_amount -> Int4,
        rarity -> Varchar,
        collection_root -> Varchar,
    }
}

diesel::table! {
    trades (id) {
        id -> Int4,
        collection_id -> Int4,
        user_id -> Int4,
        accepted -> Bool,
        accepted_order_id -> Nullable<Int4>,
        buyer_address -> Bytea,
        seller_address -> Nullable<Bytea>,
        deposited_amount -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        address -> Bytea,
        user_name -> Varchar,
    }
}

diesel::joinable!(orders -> collections (collection_id));
diesel::joinable!(orders -> trades (trade_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(trades -> collections (collection_id));
diesel::joinable!(trades -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    collections,
    orders,
    trades,
    users,
);
