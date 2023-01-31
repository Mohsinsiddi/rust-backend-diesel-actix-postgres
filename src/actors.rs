use crate::db_models::{User, Article};
use crate::db_utils::DbActor;
use crate::schema::users::{dsl::*, id as user_id};
use crate::schema::articles::{dsl::*, id as article_id};
use crate::messages::{FetchUser, FetchUserArticles,CreateArticle,CreateUser,FetchArticle};
use actix::Handler;
use diesel::{self, prelude::*};
use crate::insertables::NewArticle;
use crate::insertables::NewUser;

impl Handler<FetchUser> for DbActor {
  type Result = QueryResult<Vec<User>>;

  fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

    users.get_results::<User>(&mut conn)
  }
}

impl  Handler<FetchArticle> for DbActor {
    type Result = QueryResult<Vec<Article>>;

   fn handle(&mut self, msg: FetchArticle, ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Fetch Articles! Unable to establish a connection");
      articles.get_results::<Article>(&mut conn)
   }
}

impl Handler<FetchUserArticles> for DbActor {
  type Result = QueryResult<Vec<Article>>;

  fn handle(&mut self, msg: FetchUserArticles, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User Articles: Unable to establish connection");

    articles.filter(created_by.eq(msg.user_id)).get_results::<Article>(&mut conn)
  }
}

impl Handler<CreateArticle> for DbActor {
  type Result = QueryResult<Article>;

  fn handle(&mut self, msg: CreateArticle, ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Create User Article! Unable to establish a connection");
      
      let new_article = NewArticle {
            title:msg.title,
            content:msg.content,
            created_by:msg.created_by
      };

      diesel::insert_into(articles)
      .values(new_article)
      .returning((
        article_id,
        title,
        content,
        created_by,
        created_on.nullable()
      ))
      .get_result::<Article>(&mut conn)
  }
    
}

impl  Handler<CreateUser> for DbActor {

  type Result = QueryResult<User>;

  fn handle(&mut self, msg: CreateUser, ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Create user! Unable to establish connection");

      let new_user = NewUser {
        first_name:msg.first_name,
        last_name:msg.last_name
      };

      diesel::insert_into(users)
      .values(new_user)
      .returning((
         user_id,
         first_name,
         last_name
      ))
      .get_result::<User>(&mut conn)
  }
    
}
