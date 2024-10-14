drop table if exists course_practice cascade;
drop table if exists tutor_practice;
create table tutor_practice (
    tutor_id serial primary key,
    tutor_name varchar(200) not null,
    tutor_pic_url varchar(200) not null,
    tutor_profile varchar(2000) not null
);
create table course_practice (
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
    CONSTRAINT fk_tutor FOREIGN KEY(tutor_id) REFERENCES tutor_practice(tutor_id) ON DELETE cascade
);
grant all privileges on table tutor_practice to tutor;
grant all privileges on table course_practice to tutor;
grant all privileges on all sequences in schema public to tutor;
insert into tutor_practice (
        tutor_id,
        tutor_name,
        tutor_pic_url,
        tutor_profile
    )
values(
        1,
        'Merlene',
        'http://s3.amazon.aws.com/pic1',
        'Merlene is an experienced finance professional'
    );
insert into tutor_practice (
        tutor_id,
        tutor_name,
        tutor_pic_url,
        tutor_profile
    )
values(
        2,
        'Frank',
        'http://s3.amazon.aws.com/pic2',
        'Frank is an expert nuclear engineer'
    );
insert into course_practice (
        course_id,
        tutor_id,
        course_name,
        course_level,
        posted_time
    )
values(
        1,
        1,
        'First course',
        'Beginner',
        '2021-04-12 05:40:00'
    );
insert into course_practice (
        course_id,
        tutor_id,
        course_name,
        course_format,
        posted_time
    )
values(
        2,
        1,
        'Second course',
        'ebook',
        '2021-04-12 05:45:00'
    );