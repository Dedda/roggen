extern crate roggen;
extern crate diesel;

use self::diesel::prelude::*;
use roggen::establish_connection;
use roggen::data::models::Post;

fn main() {
    use roggen::schema::post::dsl::*;

    let connection = establish_connection();
    let results = post.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for p in results {
        println!("{}", p.title);
        println!("----------\n");
        println!("{}", p.blog);
    }
}