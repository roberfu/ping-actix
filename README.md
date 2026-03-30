# ping-actix
Ping project in Actix

## Create the project
```bash
cargo new ping-actix
cd ping-actix
```

## Add dependency
In `Cargo.toml`, add under `[dependencies]`:
```toml
actix-web = "4"
serde_json = "1"
```

## Write the server
In `src/main.rs`:
```rust
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "message": "pong" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

## Run the project
```bash
cargo run
```

## Test it
```bash
curl http://127.0.0.1:8080/ping
# Response: {"message":"pong"}
```
