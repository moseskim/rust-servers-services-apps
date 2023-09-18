## 4장 샘플 코드

### 이 저장소에 포함된 것들

이 저장소는 4장의 샘플 코드를 포함한다. 코드를 실행하려면 `chapter4/ezytutors/tutor-db` 폴더로 이동한다. 이 폴더가 프로젝트 루트이다.

### 이터레이션 1

이터레이션 1을 실행하려면 다음 명령을 실행한다.

```
cargo run --bin iter1
```

프로젝트 루트에 `.env` 파일이 있는지 확인한다. 이 파일은 데이터베이스 접근 크리덴셜을 포함한다.

노트: 이 저장소에 포함된 `.env` 파일은 학습 목적으로만 제공된다. 프러덕션 환경의 `.env`는 깃 저장소에 업로드해서는 안 된다(해당 파일은 `.gitignore`에 포함되어야 한다).

### 이터레이션 2

`iter2`가 컴파일되는지 확인한다.

```
cargo check --bin iter2
```

이터레이션 2에 대한 단위 테스트를 실행한다.

```
cargo test --bin iter2
```

### 이터레이션 3

웹 서비스를 실행한다:

```
cargo run --bin iter3
```

브라우저에서(혹은 curl을 이요해서),

`tutor-id=1`에 대한 모든 강의 정보를 얻어온다

```
http://localhost:3000/courses/1
```

새로운 강의를 개설한다.

```
curl -X POST localhost:3000/courses/ \
-H "Content-Type: application/json" \
-d '{"tutor_id":1, "course_id":4, "course_name":"Fourth course"}'
```

`tutor-id=1` 및 `course-id=4`인 강의의 세부 정보를 얻어온다.

```
http://localhost:3000/courses/1/2
```

자동화된 단위 테스트를 실행한다.

```
cargo test --bin iter3
```

단위 테스트를 반복할 때는 먼저 클린 업 스크립트를 실행한 뒤 단위 테스트를 실행해야 한다.
```
psql -U $DATABASE_USER -d ezytutors --password < $PROJECT_ROOT/iter3-test-clean.sql
cargo test --bin iter3
```

