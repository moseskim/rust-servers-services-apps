/* 테이블이 존재하면 삭제한다 */
drop table if exists ezy_course_c4;
/* 테이블을 생성한다. */
/* 노트: 마지막 필드 뒤에 쉼표를 입력하지 않는다 */
create table ezy_course_c4
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

/* 테스팅을 위한 시드 데이터를 로드한다 */
insert into ezy_course_c4
    (course_id,tutor_id, course_name,posted_time)
values(1, 1, 'First course', '2020-12-17 05:40:00');
insert into ezy_course_c4
    (course_id, tutor_id, course_name,posted_time)
values(2, 1, 'Second course', '2020-12-18 05:45:00');