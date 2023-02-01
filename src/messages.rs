use crate::db_models::{User, Trade};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;


#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Trade>>")]
pub struct FetchTrade;


#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Trade>>")]
pub struct FetchUserTrade {
  pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Trade>")]
pub struct  CreateTrade {
   pub title: String,
   pub content:String,
   pub created_by :i32,
   pub accepted_order_id :i32,
   pub deposited_amount : i32,
   pub buyer_address: String,
   pub seller_address : String,
}

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct CreateUser {
  pub address:String,
  pub user_name:String
}