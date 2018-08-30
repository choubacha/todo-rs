use super::schema::{lists, todos};

#[derive(Queryable)]
pub struct List {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name="lists"]
pub struct NewList {
    pub title: String,
}

#[derive(Queryable, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub is_completed: bool,
    pub list_id: i32,
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo {
    pub title: String,
    pub list_id: i32, // TODO: Find a better id type?
}
