use crate::db_models::{User,Collection};
use crate::db_utils::DbActor;
use crate::schema::users::{dsl::*, id as user_id};
use crate::schema::collections::{dsl::*, id as collection_id};
//use crate::schema::articles::{dsl::*, id as article_id};
use crate::messages::{FetchUser,CreateUser,FetchCollection,CreateCollection};
use actix::Handler;
use diesel::{self, prelude::*};
//use crate::insertables::NewArticle;
use crate::insertables::{NewUser,NewCollection};

impl Handler<FetchUser> for DbActor {
  type Result = QueryResult<Vec<User>>;

  fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

    users.get_results::<User>(&mut conn)
  }
}

impl Handler<FetchCollection> for DbActor {
    type Result = QueryResult<Vec<Collection>>;

    fn handle(&mut self, msg: FetchCollection, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch collection: Unable to establish connection");
        collections.get_results::<Collection>(&mut conn)
    }
}

// impl  Handler<FetchArticle> for DbActor {
//     type Result = QueryResult<Vec<Article>>;

//    fn handle(&mut self, msg: FetchArticle, ctx: &mut Self::Context) -> Self::Result {
//       let mut conn = self.0.get().expect("Fetch Articles! Unable to establish a connection");
//       articles.get_results::<Article>(&mut conn)
//    }
// }

// impl Handler<FetchUserArticles> for DbActor {
//   type Result = QueryResult<Vec<Article>>;

//   fn handle(&mut self, msg: FetchUserArticles, _ctx: &mut Self::Context) -> Self::Result {
//     let mut conn = self.0.get().expect("Fetch User Articles: Unable to establish connection");

//     articles.filter(created_by.eq(msg.user_id)).get_results::<Article>(&mut conn)
//   }
// }

// impl Handler<CreateArticle> for DbActor {
//   type Result = QueryResult<Article>;

//   fn handle(&mut self, msg: CreateArticle, ctx: &mut Self::Context) -> Self::Result {
//       let mut conn = self.0.get().expect("Create User Article! Unable to establish a connection");
      
//       let new_article = NewArticle {
//             title:msg.title,
//             content:msg.content,
//             created_by:msg.created_by
//       };

//       diesel::insert_into(articles)
//       .values(new_article)
//       .returning((
//         article_id,
//         title,
//         content,
//         created_by,
//         created_on.nullable()
//       ))
//       .get_result::<Article>(&mut conn)
//   }
    
// }

impl Handler<CreateUser> for DbActor {

  type Result = QueryResult<User>;

  fn handle(&mut self, msg: CreateUser, ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Create user! Unable to establish connection");

      let new_user = NewUser {
        address:msg.address,
        user_name:msg.user_name
      };

      diesel::insert_into(users)
      .values(new_user)
      .returning((
         user_id,
         address,
         user_name
      ))
      .get_result::<User>(&mut conn)
  }
    
}

impl Handler<CreateCollection> for DbActor {
    type Result = QueryResult<Collection>;

    fn handle(&mut self, msg: CreateCollection, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Create collection: Unable to establish connection");

        let new_collection = NewCollection {
             collection_name:msg.collection_name,
             ceiling_price:msg.ceiling_price,
             active_trades:msg.active_trades,
             total_trades:msg.total_trades,
             volume:msg.volume,
             supply:msg.supply
        };

        diesel::insert_into(collections)
        .values(new_collection)
        .returning((
          collection_id,
          collection_name,
          ceiling_price,
          active_trades,
          total_trades,
          volume,
          supply
        ))
        .get_result::<Collection>(&mut conn)
    }
}


