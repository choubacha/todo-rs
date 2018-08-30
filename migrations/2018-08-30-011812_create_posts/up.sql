-- Your SQL goes here
CREATE TABLE todos (
  id serial primary key,
  title text not null,
  is_completed boolean,
  list_id integer references lists(id)
);
