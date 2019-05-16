#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use self::models::{NewPost, Post};

pub mod schema;
pub mod models;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) -> usize {
    use schema::posts;

    let new_post = NewPost {
        title,
        body
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}