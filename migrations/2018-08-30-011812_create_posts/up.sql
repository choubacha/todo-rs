-- Your SQL goes here
CREATE TABLE todos (
  id serial primary key,
  title text not null,
  is_completed boolean not null default 'f',
  list_id integer not null references lists(id)
);
