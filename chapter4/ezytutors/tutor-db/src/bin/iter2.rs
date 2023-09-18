use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter2/handlers.rs"]
mod handlers;
#[path = "../iter2/models.rs"]
mod models;
#[path = "../iter2/routes.rs"]
mod routes;
#[path = "../iter2/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // AppState를 만든다
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    // app을 만들고 라우트를 구성한다
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    let hostname_port = env::var("SERVER_HOSTNAME_PORT").expect("SERVER_HOSTNAME_PORT is not set in .env file");

    // HTTP 서버를 시작한다
    HttpServer::new(app).bind(hostname_port).unwrap().run().await

}
