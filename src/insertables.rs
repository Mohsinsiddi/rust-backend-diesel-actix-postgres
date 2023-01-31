use crate::schema::articles;
use crate::schema::users;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable,Serialize,Clone)]
#[diesel(table_name=articles)]
pub struct NewArticle {
    pub title:String,
    pub content:String,
    pub created_by:i32
}

#[derive(Insertable,Serialize,Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub first_name:String,
    pub last_name:String,
}

