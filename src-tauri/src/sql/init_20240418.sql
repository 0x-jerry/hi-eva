-- create selection_text table

create table if not exists selection (
  id integer primary key autoincrement not null,
  selected text,
  promptName varchar(255),
  createdDate datetime not null,
  updatedDate datetime not null
);
