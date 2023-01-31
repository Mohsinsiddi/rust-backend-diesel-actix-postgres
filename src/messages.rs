use crate::db_models::{User,Collection};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Collection>>")]
pub struct FetchCollection;



// #[derive(Message)]
// #[rtype(result = "QueryResult<Vec<Article>>")]
// pub struct FetchArticle;


// #[derive(Message)]
// #[rtype(result = "QueryResult<Vec<Article>>")]
// pub struct FetchUserArticles {
//   pub user_id: i32,
// }

// #[derive(Message)]
// #[rtype(result = "QueryResult<Article>")]
// pub struct  CreateArticle {
//    pub title: String,
//    pub content:String,
//    pub created_by :i32
// }

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct CreateUser {
  pub address:String,
  pub user_name:String
}

#[derive(Message)]
#[rtype(result="QueryResult<Collection>")]
pub struct CreateCollection {
  pub collection_name:String,
  pub ceiling_price:i32,
  pub active_trades:i32,
  pub total_trades:i32,
  pub volume:i32,
  pub supply:i32 
}