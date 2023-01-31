use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse, App,
};
use serde::Deserialize;
use crate::{
    messages::{FetchUser, FetchUserArticles, CreateArticle,CreateUser,FetchArticle},
    AppState, DbActor
};
use actix::Addr;


#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub first_name: String,
    pub last_name: String,
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

#[get("/articles")]
pub async fn fetch_articles(state:Data<AppState>) -> impl Responder {
    let db : Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchArticle).await {
        Ok(Ok(info))=> HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve articles"),
    }
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("GET /users/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserArticles { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user articles"),
    }
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(state:Data<AppState>,path:Path<i32>,body:Json<CreateArticleBody>) -> impl Responder {
   let id = path.into_inner();

   let db:Addr<DbActor>  = state.as_ref().db.clone(); 

   match db.send(CreateArticle{
    title:body.title.to_string(),
    content:body.content.to_string(),
    created_by:id
   }).await {
       Ok(Ok(info))=> HttpResponse::Ok().json(info),
       _ => HttpResponse::InternalServerError().json("failed to create article")
   }
}

#[post("/users")]
pub async fn create_user(state:Data<AppState>,body:Json<CreateUserBody>) -> impl Responder {
    let db:Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateUser{
        first_name:body.first_name.to_string(),
        last_name:body.last_name.to_string()
    }).await {
        Ok(Ok(info))=> HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("failed to create user")
    }
}



