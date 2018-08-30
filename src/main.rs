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
        let list = models::NewList { title: "My list".to_string() };
        let list: models::List = diesel::insert_into(schema::lists::table)
            .values(&list)
            .get_result(&conn)
            .expect("error saving the record");

        assert_eq!(list.title, "My list");
    }
}
