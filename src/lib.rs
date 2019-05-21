#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate maud;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use crate::data::models::{Post, NewPost};

pub mod data;
pub mod schema;
pub mod server;
pub mod template;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &PgConnection, blog: &str, title: &str) -> Post {

    let new_post = NewPost {
        title: title.to_string(),
        published: false,
        blog: blog.to_string(),
    };
    use schema::post;

    diesel::insert_into(post::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn start() {
    server::ignite();
}