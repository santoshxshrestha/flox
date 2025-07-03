#![allow(unused)]
use actix_web::test::ok_service;
use askama::Template;
use serde::Deserialize;
use std::env;

use actix_files::Files;
use actix_web::web::Form;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, cookie, get, post, web};
use dotenv::dotenv;
use rand::{Rng, distr::Alphanumeric};
use sqlx::postgres::PgPoolOptions;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    id: i32,
    content: String,
    username: String,
    delete_perm: bool,
}

pub fn generate_random_token() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(19)
        .map(char::from)
        .collect()
}

#[get("/")]
async fn home(pool: web::Data<sqlx::PgPool>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let mut token = String::new();
    if let Some(cookie) = req.cookie("token") {
        token = cookie.to_string();
    }

    let rows = sqlx::query!(
        r#"
        select id,content,username,token
        from messages
        order by id desc
        "#
    )
    .fetch_all(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // First, deref web::Data<sqlx::PgPool> to Arc<sqlx::PgPool>.
    //Then, deref Arc<sqlx::PgPool> to sqlx::PgPool
    //Finally, take a reference to the sqlx::PgPool, giving you &PgPool, which is what fetch_all expects.

    let content: Vec<Message> = rows
        .into_iter()
        .map(|row| Message {
            id: row.id,
            content: row.content,
            username: row.username,
            delete_perm: { if (row.token == token) { true } else { false } },
        })
        .collect();

    let template = HomeTemplate { messages: content };
    let body = template
        .render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body(body))
}

#[derive(Deserialize)]
struct NewMessage {
    username: String,
    content: String,
}

#[post("/send")]
async fn send_message(
    pool: web::Data<sqlx::PgPool>,
    form: Form<NewMessage>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let mut token = String::new();
    let mut set_new_cookie = false;
    if let Some(cookie) = req.cookie("token") {
        token = cookie.to_string();
    } else {
        token = generate_random_token();
        set_new_cookie = true;
    }

    sqlx::query!(
        // The database treats $1, $2 as a string value, not as SQL code.
        // so the sql injection is prevented here
        "INSERT INTO messages (username, content, token) VALUES ($1, $2, $3)",
        form.username,
        form.content,
        token
    )
    .execute(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if set_new_cookie {
        let cookie = actix_web::cookie::Cookie::build("token", token)
            .path("/")
            .max_age(actix_web::cookie::time::Duration::days(30))
            .same_site(actix_web::cookie::SameSite::Lax)
            .secure(true)
            .http_only(true)
            .finish();

        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .cookie(cookie)
            .finish())
    } else {
        // Redirect back to home page to show updated messages
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .finish())
    }
}

#[derive(Deserialize)]
struct DeleteForm {
    id: i32,
}

#[post("/delete")]
async fn delete_message(
    pool: web::Data<sqlx::PgPool>,
    form: Form<DeleteForm>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let mut token = String::new();
    if let Some(cookie) = req.cookie("token") {
        token = cookie.to_string();
    } else {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let result = sqlx::query!(
        // The database treats $1, $2 as a string value, not as SQL code.
        // so the sql injection is prevented here
        "select token FROM messages WHERE id = $1",
        form.id
    )
    .fetch_optional(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(row) = result {
        if row.token == token {
            sqlx::query!("DELETE FROM messages WHERE id = $1", form.id)
                .execute(&**pool)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            Ok(HttpResponse::SeeOther()
                .append_header(("Location", "/"))
                .finish())
        } else {
            Ok(HttpResponse::Unauthorized().finish())
        }
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL mut be set.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(home)
            .service(send_message)
            .service(delete_message)
            .service(Files::new("/static", "static"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
