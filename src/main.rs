use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;

use std::env;

use actix_web_sample::handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv()?;

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(handlers::hello)
            .service(handlers::user_id)
            .service(handlers::add)
            .service(handlers::sub)
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
