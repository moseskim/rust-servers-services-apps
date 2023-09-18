## 3장 샘플 코드

### 이 저장소에 포함된 것들

#### 이 저장소는 2개의 프로젝트(basic-server와 tutor-service)를 포함한다

- 두 프로젝트를 진행하려면 이 저장소를 클론한 뒤, 해당 디렉터리로 이동한다. 이 디렉터리가 워크스페이스의 루트 폴더이다.

### basic-server 테스트 및 실행 방법

1. 한 터미널 창에서, 워크스프페이스 루트에서 다음을 실행한다.

```
cargo run -p tutor-nodb --bin basic-server
```

2. 웹 브라우저에서 다음 URI로 접근한다

```
http://localhost:3000/health
```

다음 메시지를 포함하는 웹 페이지를 확이할 수 있다.

```
"Hello. EzyTutors is alive and kicking"
```

### tutor-service 프로젝트 테스트 및 실행 방법

1. 한 터미널 창에서, 워크스페이스 루트에서 다음을 실행한다.

```
cargo run -p tutor-nodb --bin tutor-service
```

2. 새로운 터미널 창에서 다음 명령어를 실행해서 몇 개의 강의를 개설한다.

```
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my first course !"}'
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my second course !"}'
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my third course !"}'

```

3. 웹 브라우저에서 다음 URI에 접속한다.

```
localhost:3000/health
localhost:3000/courses/1
localhost:3000/courses/1/1
```

첫 번재 URI에서는 다음과 유사한 메시지를 확인할 수 있다. 방문자 카운트는 브라우저 화면을 새로고침할 때마다 증가한다.

```
"I'm good. You've already asked me  1 times"
```

두 번째 URI에서는 다음을 확인할 수 있다.

```
[{"tutor_id":1,"course_id":1,"course_name":"Hello , my first course !","posted_time":"2020-10-30T10:24:16.985954"},{"tutor_id":1,"course_id":2,"course_name":"Hello , my second course !","posted_time":"2020-10-30T10:24:17.013067"},{"tutor_id":1,"course_id":3,"course_name":"Hello , my third course !","posted_time":"2020-10-30T10:24:17.907214"}]
```

세 번째 URI에서는 다음을 확인할 수 있다.

```
{"tutor_id":1,"course_id":1,"course_name":"Hello , my first course !","posted_time":"2020-10-30T10:24:16.985954"}
```

4. tutor-service에 대한 테스트만 실행할 때는 워크스페이스 루트에서 다음을 실행한다.

```
cargo test -p tutor-nodb
```

다음과 같은 메시지가 표시되는 것을 확인할 수 있다.

```
running 3 tests
test handlers::tests::get_all_courses_success ... ok
test handlers::tests::get_one_course_success ... ok
test handlers::tests::post_course_test ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
