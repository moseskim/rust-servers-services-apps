/* Define a specific user for this book's applications */
drop user if exists truuser;
create user truuser with password 'trupwd';

/* Drop table if it already exists*/
drop table if exists ezy_course_c5;
/* Create table. */
/* 노트: 마지막 필드 뒤에 쉼표를 붙이지 않는다 */
create table ezy_course_c5
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

/* 테스팅을 위한 시드 데이터를 로드한다 
 */
insert into ezy_course_c5
    (course_id,tutor_id, course_name,posted_time)
values(1, 1, 'First course', '2021-03-17 05:40:00');
insert into ezy_course_c5
    (course_id, tutor_id, course_name,posted_time)
values(2, 1, 'Second course', '2021-03-18 05:45:00');

/* 지정한 사용자에게 privileges를 부여한다 ㄴ*/
grant all privileges on table ezy_course_c5 to truuser;
