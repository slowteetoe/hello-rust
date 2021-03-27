use actix_web::{get,App,HttpServer,HttpResponse,Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(format!("Hola mundo! (vers. {})", env!("GIT_HASH")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("0.0.0.0:1111")?
    .run()
    .await
}