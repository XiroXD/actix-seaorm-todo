use actix_seaorm_todo::config::Config;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| App::new().service(hello).wrap(Logger::default()))
        .bind((config.host, config.port))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
