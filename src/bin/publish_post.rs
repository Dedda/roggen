extern crate roggen;
extern crate diesel;

use self::diesel::prelude::*;
use self::roggen::*;
use std::env::args;
use chrono::Utc;

fn main() {
    use roggen::schema::post::dsl::{post, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let posts = diesel::update(post.find(id))
        .set(published.eq(Utc::now().naive_utc()))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published {} posts", posts);
}