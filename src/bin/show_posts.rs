extern crate roggen;
extern crate diesel;

use self::diesel::prelude::*;
use roggen::establish_connection;
use roggen::models::Post;

fn main() {
    use roggen::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}