use crate::models::{Add, HelloWorld};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/{user}/{id}")]
async fn user_id(info: web::Path<(String, u32)>) -> impl Responder {
    HttpResponse::Ok().json(HelloWorld {
        hello: "world".to_string(),
        name: info.0.to_string(),
        id: info.1,
    })
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<Add>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} + {} = {}", info.a, info.b, info.a + info.b))
}
