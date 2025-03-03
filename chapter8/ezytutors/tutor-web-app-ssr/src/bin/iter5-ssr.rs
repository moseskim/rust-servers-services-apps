#[path = "../iter5/mod.rs"]
mod iter5;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use iter5::{dbaccess, errors, handler, model, routes, state::AppState};
use actix_web::web::Data;
use routes::app_config;
use std::env;
use sqlx::postgres::PgPool;

use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // HTTP 서버를 시작한다
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    println!("Listening on: {}", &host_port);
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // AppState를 만든다
    let shared_data = web::Data::new(AppState { db: db_pool });

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter5/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .app_data(shared_data.clone())
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
