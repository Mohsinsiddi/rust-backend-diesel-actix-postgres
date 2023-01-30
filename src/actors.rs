use crate::db_models::{User,Article};
use crate::db_utils::DbActor;
use crate::schema::users::dsl::*;
use crate::messages::{FetchUser};
use actix::Handler;
use diesel::{self,prelude::*, connection};

impl Handler<FetchUser> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, msg: FetchUser, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("fetch user: unable to establish connection")
    }
}