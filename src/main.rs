use serde::Deserialize;
use uuid::Uuid;
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

#[derive(Deserialize, Debug)]
pub struct Something {
    name: String,
    nick: String,
    age: u8
}

//curl -X GET "http://127.0.0.1:8888/parameterized?name=ma&nick=Baker&age=1"
async fn parameterized(info: web::Query<Something>) -> impl Responder {
    let my_uuid:Uuid = Uuid::new_v4();
    println!("new uuid:{}", my_uuid);
    println!("{:?}", info);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html ; charset=utf-8")
        .body( format!("call her {} {}", info.name, info.nick))
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/hello", web::get().to(greet))
            .route("/home", web::get().to(home))
            .route("/parameterized", web::get().to(parameterized))
    })
    .bind("127.0.0.1:8888")?
        .run()
        .await

}

