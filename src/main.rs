#![allow(unused)]
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use askama::Template;
use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;
use std::string::String;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home;

#[get("/")]
pub async fn home() -> impl Responder {
    let template = Home {};
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL mut be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap();

    let rows = sqlx::query("select * from messages")
        .fetch_all(&pool)
        .await
        .unwrap();
    for row in rows {
        let id: i32 = row.get("id");
        let content: String = row.get("content");
        let username: String = row.get("username");
        let createdat: String = row.get("created_at");
        println!(
            "id {}  content {} username {} createdat{}",
            id, content, username, createdat
        );
    }

    HttpServer::new(|| App::new().service(home))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
