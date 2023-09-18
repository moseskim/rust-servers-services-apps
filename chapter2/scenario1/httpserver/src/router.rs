use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            // GET 요청이면
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    // URI를 파싱한다
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        // 라우트가 /api로 시작하면 웹 서비스를 호출한다
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        // 그렇지 않으면 정적 페이지 핸들러를 호출한다
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            // 메서드가 GET 요청이 아니면 404 페이지를 반환한다
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
