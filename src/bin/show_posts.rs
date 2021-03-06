extern crate diesel;
extern crate roggen;

use std::ops::Deref;

use self::diesel::prelude::*;
use roggen::data::connections::get_pool;
use roggen::data::models::Post;

fn main() {
    use roggen::schema::post::dsl::*;

    let pooled = get_pool().get().expect("Got no connection from pool");
    let connection = pooled.deref();
    let results = post
        .filter(published.is_not_null())
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for p in results {
        println!("{}", p.title);
        println!("----------\n");
        println!("{}", p.blog);
    }
}
