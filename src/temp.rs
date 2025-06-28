
#![allow(unused)]
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, get, web};
use askama::Template;
use dotenv::dotenv;
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Debug)]
struct Message {
    id: i32,
    content: String,
    username: String,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    messages: Vec<Message>,
}

#[get("/")]
async fn home(pool: web::Data<sqlx::PgPool>) -> actix_web::Result<HttpResponse> {
    let rows = a
    // let rows = sqlx::query!("select * from messages");
    // let rows = sqlx::query("SELECT id, content, username FROM messages ORDER BY created_at DESC")
    //     .fetch_all(pool.get_ref())
    //     .await
    //     .map_err(actix_web::error::ErrorInternalServerError)?;
    //
    // let messages = rows
    //     .into_iter()
    //     .map(|row| Message {
    //         id: row.get("id"),
    //         content: row.get("content"),
    //         username: row.get("username"),
    //     })
    //     .collect();
    //
    let template = HomeTemplate { messages };
    let body = template
        .render()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(home)
            .service(Files::new("/static", "static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
