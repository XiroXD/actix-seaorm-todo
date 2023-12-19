use actix_seaorm_todo::{config::Config, routes::todo_routes::hello};
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| App::new().service(hello).wrap(Logger::default()))
        .bind((config.host, config.port))?
        .run()
        .await
}
