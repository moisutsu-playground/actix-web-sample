use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;

use serde::Serialize;

use std::env;

#[derive(Serialize)]
struct HelloWorld {
    hello: String,
    name: String,
    id: u32,
}

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

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv()?;

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(user_id)
            .wrap(middleware::Logger::default())
    });

    env_logger::init();

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(format!("{}:{}", env::var("HOST")?, env::var("PORT")?))?
    };

    server.run().await?;
    Ok(())
}
