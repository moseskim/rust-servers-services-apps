use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        // 유입되는 HTTP 요청의 각 행을 읽는다
        for line in req.lines() {
            // 해당 행이 요청 행이면 process_req_line() 함수를 호출한다
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            // 해당 행이 헤더 행이면 process_header_line() 함수를 호출한다
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            //  해당 행이 빈 행이면 아무 것도 하지 않는다
            } else if line.len() == 0 {
                // 위의 어떤 행도 아니면 메시지 바디로 취급한다
            } else {
                parsed_msg_body = line;
            }
        }
        // 유입된 HTTP 요청을 HttpReqeust 구조체로 파싱한다
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    // 요청 행을 공백으로 구분된 개별 덩어리로 파싱한다
    let mut words = s.split_whitespace();
    // 요청 행의 첫 번째 부분에서 HTTP 메서드를 추출한다
    let method = words.next().unwrap();
    // 요청 행의 두 번째 부분에서 리소스(URI/URL)을 추출한다
    let resource = words.next().unwrap();
    // 요청 행의 세 번째 부분에서 HTTP 버전을 추출한다
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}
fn process_header_line(s: &str) -> (String, String) {
    // 헤더 행을 구분자(';')로 나눠진 단어로 파싱한다
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    // 헤도의 키 부분을 추출한다
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    // 헤더의 값 부분을 추출한다
    if let Some(v) = header_items.next() {
        value = v.to_string()
    }

    (key, value)
}
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}
impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}
impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
