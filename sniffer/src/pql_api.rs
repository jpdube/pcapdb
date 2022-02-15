use actix_web::{web, App, HttpServer};
use actix_web::Responder;
use std::thread;
use serde::{Serialize, Deserialize}; 

struct ApiControl {
    stop: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PqlQuery {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResp {
    pub query: String,
    pub response: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}


#[actix_rt::main]
async fn main_api() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            // .route("/users", web::get().to(get_users))
            // .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/query", web::post().to(exec_query))
            .route("/login", web::post().to(login))
            // .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


pub async fn login(item: web::Json<User>) -> impl Responder {


    format!("Username {}, Password {}", item.username, item.password)
}

pub async fn exec_query(item: web::Json<PqlQuery>) -> impl Responder {
    let resp = QueryResp {
        query: item.query.clone(),
        response: "This is the response!!!".to_string()
    };

    format!("Query {:?}", resp)
}


pub fn start () {

    thread::spawn(|| {
        main_api();
    });
}
