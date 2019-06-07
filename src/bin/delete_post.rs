extern crate diesel;
extern crate roggen;

use self::diesel::prelude::*;
use self::roggen::*;
use std::env::args;

fn main() {
    use roggen::schema::post::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(post.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
