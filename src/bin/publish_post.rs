extern crate roggen;
extern crate diesel;

use self::diesel::prelude::*;
use self::roggen::*;
use std::env::args;

fn main() {
    use roggen::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published {} posts", post);
}