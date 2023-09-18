## 7장 러스트를 사용한 서버-사이드 웹앱 도입

#### 예제 1: 정적 웹 페이지 제공하기

`tutor-web-app-ssr`의 루트 폴더에서 다음을 실행한다.

```
cargo run --bin iter1
```

브라우저에서 다음 URL로 방문한다.

```
http://localhost:8080/static/static-web-page.html
```

#### 예제 2: 동적 웹 페이지 제공하기

```
cargo run --bin iter1
```

브라우저에서 다음 URL로 방문한다.

````
http://localhost:8080/
````

#### 예제 3: 폼을 통해 사용자 입력을 받는다

```
cargo run --bin iter2
```

브라우저에서 다음 URL로 방문한다.

```
 http://localhost:8080/
```


#### Example 4: Displaying a list with templates

```
cargo run --bin iter3
```

From a browser: 
```
http://localhost:8080/tutors
```

#### Example 5: Writing unit and integration tests

```
cargo test --bin iter2
```

#### Example 6: Connecting to backend web service

From root folder of __tutor-web-app-ssr__, run:
```
cargo run --bin iter4
```

From root folder of __tutor-web-service__, run:
```
cargo run --bin iter5
```
From a browser: 
```
localhost:8080/tutors
```

#### Debugging tip
In case of difficulty in building the server due to missing/inconsistent dependencies, delete the ```cargo.lock``` file and ```target``` folder and rebuild.