use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod index;
mod predict;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index::index))
            .route("/upload", web::post().to(predict::upload_file))
    })
    .bind(&server_address)?
    .run()
    .await
}
