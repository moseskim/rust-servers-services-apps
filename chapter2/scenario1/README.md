## 2장 샘플 코드

### 이 저장소에 포함된 것들

#### 이 저장소는 두 개의 프로젝트(TPC server/client와 HTTP Server)를 포함하고 있다.

- 두 프로젝트를 진행하려면 이 저장소를 클론한 뒤, 해당 디렉터리로 이동한다. 이 디렉터리가 워크스페이스의 루트 폴더이다.

### 워크스페이스 루트 폴더에서 TCP server/client 테스트 및 실행 방법

1. 하나의 터미널 창에서 다음을 실행한다.

```
cargo run -p tcpserver
```


2. 다른 터미널 창에서 다음을 실행한다.

```
cargo run -p tcpclient
```

클라이언트가 실행되고 있는 터미널 창에서 다음과 같은 메시지를 확인할 수 있다.

```
Got response from server:"Hello"
```

3. 단순히 테스트만 실행하려면 다음을 실행한다.

```
cargo test -p http
```

터미널에서 다음과 같은 메시지를 확인할 수 있다.

```
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### HTTP Servedr 프로젝트 테스트 및 실행 방법

1. 터미널 창에서 다음을 실행한다.

```
cargo run -p httpserver
```

2. 웹 브라우저에서 다음 URI에 접근한다.

```
localhost:3000
localhost:3000/health
localhost:3000/api/shipping/orders
localhost:3000/invalid-page
```

처음 2개 URI에서는 해당하는 HTML 페이지가 제공되는 것을 볼 수 있다.

세 번째 URI에서는 서버에서 반환되는 JSON 데이터를 볼 수 있다.

마지막 URI에서는 404 에러 페이지를 볼 수 있다.
