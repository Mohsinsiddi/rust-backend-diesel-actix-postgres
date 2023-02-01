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
    trades (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        created_by -> Int4,
        accepted_order_id -> Int4,
        deposited_amount -> Int4,
        buyer_address -> Varchar,
        seller_address -> Varchar,
        created_on -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        address -> Varchar,
        user_name -> Varchar,
    }
}

diesel::joinable!(trades -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    collections,
    trades,
    users,
);
