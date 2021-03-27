use actix_web::App;
use actix_web::HttpServer;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{get};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hola mundo!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("127.0.0.1:1111")?
    .run()
    .await
}