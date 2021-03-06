use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/", ".")
        .index_file("index.html")
        .show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}