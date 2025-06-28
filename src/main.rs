use askama::Template;
use serde::Deserialize;
use std::env;

use actix_files::Files;
use actix_web::web::Form;
use actix_web::{App, HttpResponse, HttpServer, get, post, web};
use dotenv::dotenv;
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
}

#[get("/")]
async fn home(pool: web::Data<sqlx::PgPool>) -> actix_web::Result<HttpResponse> {
    let rows = sqlx::query!(
        r#"
        select id,content,username 
        from messages
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
) -> actix_web::Result<HttpResponse> {
    sqlx::query!(
        // The database treats $1, $2 as a string value, not as SQL code.
        // so the sql injection is prevented here
        "INSERT INTO messages (username, content) VALUES ($1, $2)",
        form.username,
        form.content,
    )
    .execute(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Redirect back to home page to show updated messages
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
}

#[derive(Deserialize)]
struct DeleteForm {
    id: i32,
}

#[post("/delete")]
async fn delete_message(
    pool: web::Data<sqlx::PgPool>,
    form: Form<DeleteForm>,
) -> actix_web::Result<HttpResponse> {
    sqlx::query!(
        // The database treats $1, $2 as a string value, not as SQL code.
        // so the sql injection is prevented here
        "DELETE FROM messages WHERE id = $1",
        form.id
    )
    .execute(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Redirect back to home page to show updated messages
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
