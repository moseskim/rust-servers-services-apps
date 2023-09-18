// 모듈 임포트
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// 라우트를 구성한다 <1>
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// 핸드러를 구성한다 <2>
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

// HTTP 서버를 인스턴스화하고 실행한다
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // app을 만들고 라우트를 구성한다 <3>
    let app = move || App::new().configure(general_routes);

    // HTTP 서버를 시작한다 <4>
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
} 