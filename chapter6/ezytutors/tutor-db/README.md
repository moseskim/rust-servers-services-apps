#### 서버를 실행하는 단계

1. 본문에서 설명한 과정을 다라 `database`와 비밀번호를 가진 `user`를 생성한다

2. 데이터베이싀 해당 사용자에게 권한을 부여한다

3. `.env` 파일이 올바른 `DATABASE_URL`과 `HOST_PORT` 환경 변수를 사용해서 구성되었는지 확인한다.

4. `postgres` 데이터베이스 안에서 `tutor`과 `course` 테이블 생성을 위한 스크립트 데이터베이스를 실행하고, 시드 데이터를 업로드한다.

5. `tutor_db`의 루트 폴더에서 다음을 실해한다.

[source,shell]

```
cargo check
cargo run --bin iter5
```

6. 자동화된 테스트를 실행한다.

[source,shell]

```
cargo test
``````

7. 브라우저에서 다음 주소에 방문한다.

[source,shell]

``````
http://localhost:3000/tutors/       // 모든 강사 목록을 얻는다
http://localhost:3000/tutors/2      // 단일 강사의 상세 정보를 얻는다
http://localhost:3000/courses/1     // 단일 강사의 모든 강의 정보를 얻는다
http://localhost:3000/courses/1/2   // 단일 강사의 하나의 강의 정보를 얻는다
```

8. 강사와 강의 생성, 업데이트, 삭제에 관한 추가적인 설명들은 해당 장의 내용을 참조한다.

#### 디버깅 팁

디펜턴시 누락/비일관성으로 인해 서버를 구축하는 데 문제가 있다면 `cargo.lock` 파일과 `target` 폴더를 삭제한 뒤 재빌드하라.