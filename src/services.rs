use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse, App,
};
use serde::Deserialize;
use crate::{
    messages::{FetchUser,CreateUser,FetchCollection,CreateCollection},
    AppState, DbActor
};
use actix::Addr;


// #[derive(Deserialize)]
// pub struct CreateArticleBody {
//     pub title: String,
//     pub content: String,
// }

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub address: String,
    pub user_name: String,
}

#[derive(Deserialize)]
pub struct CreateCollectionBody {
    pub collection_name:String,
    pub ceiling_price:i32,
    pub active_trades:i32,
    pub total_trades:i32,
    pub volume:i32,
    pub supply:i32 
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

#[get("/collections")]
pub async fn fetch_collections(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchCollection).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No collections found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve collections"),
    }
}

// #[get("/articles")]
// pub async fn fetch_articles(state:Data<AppState>) -> impl Responder {
//     let db : Addr<DbActor> = state.as_ref().db.clone();

//     match db.send(FetchArticle).await {
//         Ok(Ok(info))=> HttpResponse::Ok().json(info),
//         _ => HttpResponse::InternalServerError().json("Unable to retrieve articles"),
//     }
// }

// #[get("/users/{id}/articles")]
// pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
//     let id: i32 = path.into_inner();
//     // format!("GET /users/{id}/articles")

//     let db: Addr<DbActor> = state.as_ref().db.clone();

//     match db.send(FetchUserArticles { user_id: id }).await {
//         Ok(Ok(info)) => HttpResponse::Ok().json(info),
//         Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles for user {id}")),
//         _ => HttpResponse::InternalServerError().json("Unable to retrieve user articles"),
//     }
// }

// #[post("/users/{id}/articles")]
// pub async fn create_user_article(state:Data<AppState>,path:Path<i32>,body:Json<CreateArticleBody>) -> impl Responder {
//    let id = path.into_inner();

//    let db:Addr<DbActor>  = state.as_ref().db.clone(); 

//    match db.send(CreateArticle{
//     title:body.title.to_string(),
//     content:body.content.to_string(),
//     created_by:id
//    }).await {
//        Ok(Ok(info))=> HttpResponse::Ok().json(info),
//        _ => HttpResponse::InternalServerError().json("failed to create article")
//    }
// }

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

#[post("/collections")]
pub async fn create_collection(state:Data<AppState>,body:Json<CreateCollectionBody>) -> impl Responder {
    let db:Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateCollection{
        collection_name:body.collection_name.to_string(),
        ceiling_price:body.ceiling_price,
        active_trades:body.active_trades,
        total_trades:body.active_trades,
        volume:body.volume,
        supply:body.supply
    }).await {
        Ok(Ok(info))=> HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("failed to create collection")
    }
}



