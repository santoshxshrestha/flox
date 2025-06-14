mod routes;
use actix_web::App;
use actix_web::HttpServer;
use routes::home::home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
