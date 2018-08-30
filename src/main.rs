#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn establish_connection() -> PgConnection {
    use std::env;

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod some_toy_tests {
    use super::*;

    #[test]
    fn test_some_things() {
        let conn = establish_connection();
        let result = conn.transaction::<_, diesel::result::Error, _>(|| {
            let list = models::NewList { title: "My list".to_string() };
            let list: models::List = diesel::insert_into(schema::lists::table)
                .values(&list)
                .get_result(&conn)
                .expect("error saving the record");

            assert_eq!(list.title, "My list");
            Ok(())
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_inserting_todos() {
        let conn = establish_connection();
        let result = conn.transaction::<_, diesel::result::Error, _>(|| {
            let list = models::NewList { title: "My list".to_string() };
            let list: models::List = diesel::insert_into(schema::lists::table)
                .values(&list)
                .get_result(&conn)
                .expect("error saving the record");

            assert_eq!(list.title, "My list");

            let todo = models::NewTodo {
                title: "My Todo".to_string(),
                list_id: list.id,
            };
            let todo: models::Todo = diesel::insert_into(schema::todos::table)
                .values(&todo)
                .get_result(&conn)
                .expect("error saving todo");

            assert_eq!(todo.title, "My Todo");
            assert_eq!(todo.list_id, list.id);
            assert!(todo.is_completed, false);

            Ok(())
        });

        assert!(result.is_ok());
    }
}
