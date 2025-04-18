-- create selection_text table

create table if not exists selection_text (
    id integer primary key autoincrement not null,
    selected text
);