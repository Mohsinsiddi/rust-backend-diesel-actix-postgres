use crate::schema::trades;
use crate::schema::users;
use crate::schema::collections;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable,Serialize,Clone)]
#[diesel(table_name=collections)]
pub struct NewCollection {
    pub collection_name: String,
    pub ceiling_price: i32,
    pub active_trades: i32,
    pub total_trades: i32,
    pub volume: i32,
    pub supply: i32,
}

#[derive(Insertable,Serialize,Clone)]
#[diesel(table_name=trades)]
pub struct NewTrade {
    pub title:String,
    pub content:String,
    pub created_by:i32,
    pub accepted_order_id :i32,
    pub deposited_amount : i32,
    pub buyer_address: String,
    pub seller_address : String,
}

#[derive(Insertable,Serialize,Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub address:String,
    pub user_name:String,
}

