/* 테이블이 존재하면 삭제한다 */

drop table if exists ezy_course_c6;

/* 테이블을 생성한다. */
/* 노트: 마지막 필드 뒤에 쉼표를 붙이지 않는다 */

create table ezy_course_c6
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    course_description varchar(2000),
    course_format varchar(30),
    course_structure varchar(200),
    course_duration varchar(30),
    course_price INT,
    course_language varchar(30),
    course_level varchar(30),
    posted_time TIMESTAMP default now()

);

grant all privileges on table ezy_course_c6 to truuser;
grant all privileges on all sequences in schema public to truuser;
