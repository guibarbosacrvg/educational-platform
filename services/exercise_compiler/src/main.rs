mod lib;
use actix_cors::Cors;
use actix_web::{web, http, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Permitir origem do frontend
            .allowed_methods(vec!["GET", "POST"]) // Métodos permitidos
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .service(web::resource("/run/{language}").route(web::post().to(lib::compilers::run_code)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
