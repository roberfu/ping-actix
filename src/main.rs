use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "message": "pong" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(ping)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}