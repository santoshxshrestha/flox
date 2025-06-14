mod routes;
#![allow(unused)]
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use askama::Template;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
