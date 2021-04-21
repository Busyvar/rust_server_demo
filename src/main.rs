use actix_web::{web, Responder, App, HttpResponse,HttpServer};
use actix_web::http::{StatusCode};

async fn home() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html ; charset=utf-8")
        .body(include_str!("../templates/index.html"))
}

async fn greet() -> impl Responder {
    const MSG: &str = "Hello!";
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html ; charset=utf-8")
        .body(MSG)
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/hello", web::get().to(greet))
            .route("/home", web::get().to(home))
    })
    .bind("127.0.0.1:8888")?
        .run()
        .await

}

