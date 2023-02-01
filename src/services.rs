use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse, App,
};
use serde::Deserialize;
use crate::{
    messages::{FetchUser, FetchUserTrade, CreateTrade,CreateUser,FetchTrade},
    AppState, DbActor
};
use actix::Addr;


#[derive(Deserialize)]
pub struct CreateTradeBody {
    pub title: String,
    pub content: String,
    pub accepted_order_id :i32,
    pub deposited_amount : i32,
    pub buyer_address: String,
    pub seller_address : String,
}

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub address: String,
    pub user_name: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

#[get("/trades")]
pub async fn fetch_articles(state:Data<AppState>) -> impl Responder {
    let db : Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchTrade).await {
        Ok(Ok(info))=> HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve articles"),
    }
}

#[get("/users/{id}/trades")]
pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("GET /users/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserTrade { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user articles"),
    }
}

#[post("/users/{id}/trades")]
pub async fn create_user_article(state:Data<AppState>,path:Path<i32>,body:Json<CreateTradeBody>) -> impl Responder {
   let id = path.into_inner();

   let db:Addr<DbActor>  = state.as_ref().db.clone(); 

   match db.send(CreateTrade{
    title:body.title.to_string(),
    content:body.content.to_string(),
    created_by:id,
    accepted_order_id:body.accepted_order_id,
    deposited_amount:body.deposited_amount,
    buyer_address:body.buyer_address.to_string(),
    seller_address:body.seller_address.to_string()
   }).await {
       Ok(Ok(info))=> HttpResponse::Ok().json(info),
       _ => HttpResponse::InternalServerError().json("failed to create article")
   }
}

#[post("/users")]
pub async fn create_user(state:Data<AppState>,body:Json<CreateUserBody>) -> impl Responder {
    let db:Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateUser{
        address:body.address.to_string(),
        user_name:body.user_name.to_string()
    }).await {
        Ok(Ok(info))=> HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("failed to create user")
    }
}



