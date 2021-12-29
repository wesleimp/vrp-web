use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};

mod solve;

/// This handler uses json extractor
async fn index() -> HttpResponse {
    HttpResponse::Ok().json("health") // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/_health").route(web::get().to(index)))
            .service(web::resource("/solve").route(web::post().to(solve::solution)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
