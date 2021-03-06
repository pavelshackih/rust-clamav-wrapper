use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
