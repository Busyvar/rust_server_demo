use actix_web::{get, Responder, App, HttpResponse,HttpServer};
use actix_web::http::{StatusCode};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
    .content_type("text/html ; charset=utf-8")
    .body(include_str!("../templates/index.html"))
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind("127.0.0.1:8888")?
        .run()
        .await

}

