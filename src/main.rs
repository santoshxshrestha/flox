#![allow(unused)]
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use askama::Template;
use dotenv::dotenv;
use reqwest;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home;

#[get("/")]
pub async fn home() -> impl Responder {
    let template = Home;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
