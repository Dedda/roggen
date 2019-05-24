#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate maud;
#[macro_use]
extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate uebersetzt;

use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use rocket::http::Cookies;
use crate::data::models::{Post, NewPost, Heading, NewHeading, TextSection, NewTextSection};

pub mod data;
pub mod i18n;
pub mod img;
pub mod schema;
pub mod server;
pub mod template;

pub fn get_lang(cookie: Cookies) -> String {
    let languages = vec!["en", "de"];
    match cookie.get("selected-language") {
        Some(c) => if languages.contains(&c.value()) {
            c.value().to_string()
        } else {
            "en".to_string()
        },
        None => "en".to_string(),
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &PgConnection, blog: &str, title: &str) -> Post {

    let new_post = NewPost::new(title.to_string(), blog.to_string());
    use schema::post;

    diesel::insert_into(post::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_heading(conn: &PgConnection, post: i32, index: i32, text: &str, size: i32) -> Heading {
    let new_heading = NewHeading {
        post,
        section_index: index,
        heading_text: text.to_string(),
        heading_size: size,
    };
    use schema::heading;
    diesel::insert_into(heading::table)
        .values(&new_heading)
        .get_result(conn)
        .expect("Error saving new heading")
}

pub fn create_text(conn: &PgConnection, post: i32, index: i32, text: &str) -> TextSection {
    let new_text = NewTextSection {
        post,
        section_index: index,
        section_text: text.to_string(),
    };
    use schema::text_section;
    diesel::insert_into(text_section::table)
        .values(&new_text)
        .get_result(conn)
        .expect("Error saving new text")
}

pub fn start() {
    server::ignite();
}