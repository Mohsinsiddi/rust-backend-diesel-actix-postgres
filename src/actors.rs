use crate::db_models::{User, Trade,Collection};
use crate::db_utils::DbActor;
use crate::schema::collections::{dsl::*, id as collection_id};
use crate::schema::users::{dsl::*, id as user_id};
use crate::schema::trades::{dsl::*, id as trade_id};
use crate::messages::{FetchUser, FetchUserTrade,CreateTrade,CreateUser,FetchTrade,FetchCollection,CreateCollection};
use actix::Handler;
use diesel::{self, prelude::*};
use crate::insertables::NewTrade;
use crate::insertables::NewUser;
use crate::insertables::NewCollection;

impl Handler<FetchUser> for DbActor {
  type Result = QueryResult<Vec<User>>;

  fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

    users.get_results::<User>(&mut conn)
  }
}

impl  Handler<FetchTrade> for DbActor {
    type Result = QueryResult<Vec<Trade>>;

   fn handle(&mut self, msg: FetchTrade, ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Fetch Articles: Unable to establish a connection");
      trades.get_results::<Trade>(&mut conn)
   }
}

impl Handler<FetchCollection> for DbActor {
    type Result = QueryResult<Vec<Collection>>;

    fn handle(&mut self, msg: FetchCollection, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch Collection: Unable to establish a connection");
        collections.get_results::<Collection>(&mut conn)
    }
}

impl Handler<FetchUserTrade> for DbActor {
  type Result = QueryResult<Vec<Trade>>;

  fn handle(&mut self, msg: FetchUserTrade, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User Articles: Unable to establish connection");

    trades.filter(created_by.eq(msg.user_id)).get_results::<Trade>(&mut conn)
  }
}

impl Handler<CreateTrade> for DbActor {
  type Result = QueryResult<Trade>;

  fn handle(&mut self, msg: CreateTrade, ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Create User Article! Unable to establish a connection");
      
      let new_article = NewTrade {
            title:msg.title,
            content:msg.content,
            created_by:msg.created_by,
            accepted_order_id:msg.accepted_order_id,
            deposited_amount:msg.deposited_amount,
            buyer_address:msg.buyer_address,
            seller_address:msg.seller_address
      };

      diesel::insert_into(trades)
      .values(new_article)
      .returning((
        trade_id,
        title,
        content,
        created_by,
        accepted_order_id,
        deposited_amount,
        buyer_address,
        seller_address,
        created_on.nullable()
      ))
      .get_result::<Trade>(&mut conn)
  }
    
}

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
      let mut conn = self.0.get().expect("Create Collection: Unable to establish connection");

      let new_collection = NewCollection {
        collection_name:msg.collection_name,
        ceiling_price:msg.ceiling_price,
        active_trades:msg.active_trades,
        total_trades:msg.total_trades,
        volume:msg.volume,
        supply:msg.volume
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
