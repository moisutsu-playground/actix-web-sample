use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;

use serde::Serialize;

use std::env;

#[derive(Serialize)]
struct HelloWorld {
    hello: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(HelloWorld {
        hello: "world".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv()?;

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(index)
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
