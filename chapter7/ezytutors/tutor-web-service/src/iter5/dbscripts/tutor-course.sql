/* 테이블이 존재하면 삭제한다 */

drop table if exists ezy_course_c6;
drop table if exists ezy_tutor_c6;

/* 테이블을 생성한다. */
/* 노트: 마지막 필드 뒤에 쉼표를 붙이지 않는다 */

create table ezy_tutor_c6 (
    tutor_id serial primary key,
    tutor_name varchar(200) not null,
    tutor_pic_url varchar(200) not null,
    tutor_profile varchar(2000) not null
);

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
    posted_time TIMESTAMP default now(),
    CONSTRAINT fk_tutor
    FOREIGN KEY(tutor_id)
        REFERENCES ezy_tutor_c6(tutor_id)
);

grant all privileges on table ezy_tutor_c6 to truuser;
grant all privileges on table ezy_course_c6 to truuser;


/* 테스팅을 위한 시드 데이터를 로드한다 
 */
insert into ezy_tutor_c6(tutor_id, tutor_name, tutor_pic_url,tutor_profile)
values(1,'Merlene','http://s3.amazon.aws.com/pic1','Merlere is an experienced finance professional');

insert into ezy_tutor_c6(tutor_id, tutor_name, tutor_pic_url,tutor_profile)
values(2,'Frank','http://s3.amazon.aws.com/pic2','Frank is an expert nuclear engineer');


insert into ezy_course_c6
    (course_id,tutor_id, course_name,course_level, posted_time)
values(1, 1, 'First course', 'Beginner' , '2021-04-12 05:40:00');
insert into ezy_course_c6
    (course_id, tutor_id, course_name, course_format, posted_time)
values(2, 2, 'Second course', 'ebook', '2021-04-12 05:45:00');

