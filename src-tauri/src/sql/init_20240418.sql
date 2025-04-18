-- create selection_text table

create table if not exists selection (
    id integer primary key autoincrement not null,
    selected text,
    created_date integer not null,
    updated_date integer not null
);