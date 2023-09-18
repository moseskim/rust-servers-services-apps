## 12장

### 서버 실행하기

1. 책의 내용을 참조해 `database`와 비밀번호를 가진 `user`를 생성한다

2. 해당 데이터베이스의 해당 사용자에게 권한을 부여한다.

3. `.env` 파일에 올바른 `DATABASE_URl`과 `HOST_PORT` 환경 변수가 설정되어 있는지 확인한다.

4. Execute the database scripts for ```tutor``` and ```course``` table creation and seed data upload, in the ```postgres``` database.

4. `postgres` 데이터베이스 안에 `tutor`, `course` 테이블을 생성하고 시드 데이터를 업로드하기 위한 데이터베이스 스크립트를 실행한다.

5. `tutor-db`의 루트 폴더에서 다음을 실행한다.

[source,shell]

```
cargo check
cargo run --bin iter5
```

6. 자동화된 테스트를 실행한다.

[source,shell]

```
cargo test
```

7. 브라우저에서 다음 URL로 방문한다.

[source,shell]

```
http://localhost:3000/tutors/       // 모든 강사의 목록을 얻는다
http://localhost:3000/tutors/2      // 단일 강사의 상세 정보를 얻는다
http://localhost:3000/courses/1     // 단일 강사의 모든 강의를 얻는다
http://localhost:3000/courses/1/2   // 단일 강사의 한 강의싀 세부 정보를 얻는다
```

8. 강사 및 강의 생성, 업데이트, 삭제에 관한 자세한 설명은 12장의 내용을 참조한다.

### 디버깅 팁

In case of difficulty in building the server due to missing/inconsistent dependencies, delete the ```cargo.lock``` file and ```target``` folder and rebuild.

디펜던시 누락 및 미일치로 인해 서버를 빌드하는 데 문제가 발생한다면 `cargo.lock` 파일과 `target` 폴더를 지우고 재빌드한다.
