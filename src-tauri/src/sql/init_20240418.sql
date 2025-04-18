-- create selection_text table

create table if not exists selection (
    id integer primary key autoincrement not null,
    selected text,
    prompt_name varchar(255),
    count int default 0,
    created_date datetime not null,
    updated_date datetime not null
);