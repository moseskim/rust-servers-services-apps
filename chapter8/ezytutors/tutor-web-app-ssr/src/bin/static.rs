
// Listing 8.x - 시작

use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Listing 8.x - 끝
