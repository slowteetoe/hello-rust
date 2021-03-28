use actix_web::{get,middleware,App,HttpServer,HttpResponse,Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(format!("Hola mundo! (vers. {})", env!("GIT_HASH")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello)
    })
    .bind("0.0.0.0:1111")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    extern crate spectral;
    use spectral::prelude::*;
    use super::*;
    use std::str;
    use actix_web::{http, test};
    use regex::Regex;

    #[actix_rt::test]
    async fn test_hello_route() {
        let mut app = test::init_service(App::new().service(hello)).await;

        let req = test::TestRequest::with_uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
        let bytes = test::read_body(resp).await;
        let body = str::from_utf8(&bytes).unwrap();
        // not liking this, if we don't do something like spectral below, we don't see a decent error message
        assert_that(&body).starts_with(r#"Hola mundo! (vers. "#);

        // even though this regex is better, we do not get an indication of what the non-matching string was, if it fails
        let re = Regex::new(r#"Hola mundo! \(vers. [a-f0-9]{40}\)"#).unwrap();
        assert!(re.is_match(&body));
    }

}