mod handler;
mod router;
mod server;
use server::Server;
fn main() {
    // 서버를 시작한다
    let server = Server::new("localhost:3000");
    // 서버를 실행한다
    server.run();
}
