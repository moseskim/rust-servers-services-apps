
drop table if exists ezyweb_user;

/* 테이블을 생성한다. */
/* 노트: 마지막 필드 뒤에 쉼표를 붙이지 않는다 */

create table ezyweb_user
(
    username varchar(20) primary key,
    tutor_id INT,    
    user_password CHAR(100) not null
);

grant all privileges on table ezyweb_user to truuser;
grant all privileges on all sequences in schema public to truuser;
