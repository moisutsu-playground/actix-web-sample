use crate::models::{Add, HelloWorld, Sub};
use actix_web::{get, web::Path, web::Query, HttpResponse, Responder};

#[get("/{user}/{id}")]
async fn user_id(Path((user, id)): Path<(String, u32)>) -> impl Responder {
    HttpResponse::Ok().json(HelloWorld {
        hello: "world".to_string(),
        name: user,
        id: id,
    })
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/add/{a}/{b}")]
async fn add(info: Path<Add>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} + {} = {}", info.a, info.b, info.a + info.b))
}

#[get("/sub")]
async fn sub(info: Query<Sub>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} - {} = {}", info.a, info.b, info.a - info.b))
}
