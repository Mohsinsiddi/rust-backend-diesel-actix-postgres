// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::DateTime;
use chrono::offset::Utc;

use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Collection {
    pub id: i32,
    pub collection_name: String,
    pub ceiling_price: i32,
    pub active_trades: i32,
    pub total_trades: i32,
    pub volume: i32,
    pub supply: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Trade {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_by: i32,
    pub accepted_order_id: i32,
    pub deposited_amount: i32,
    pub buyer_address: String,
    pub seller_address: String,
    #[serde(skip_serializing)]
    pub created_on: Option<DateTime<Utc>>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub address: String,
    pub user_name: String,
}

