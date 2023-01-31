//use crate::schema::articles;
use crate::schema::users;
use crate::schema::collections;
use diesel::Insertable;
use serde::Serialize;

// #[derive(Insertable,Serialize,Clone)]
// #[diesel(table_name=articles)]
// pub struct NewArticle {
//     pub title:String,
//     pub content:String,
//     pub created_by:i32
// }

#[derive(Insertable,Serialize,Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub address:String,
    pub user_name:String,
}

#[derive(Insertable,Serialize,Clone)]
#[diesel(table_name=collections)]
pub struct NewCollection {
    pub collection_name:String,
    pub ceiling_price:i32,
    pub active_trades:i32,
    pub total_trades:i32,
    pub volume:i32,
    pub supply:i32 
}

