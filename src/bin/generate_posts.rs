extern crate diesel;
extern crate rand;
extern crate roggen;

use self::roggen::*;
use diesel::PgConnection;
use rand::Rng;

fn main() {
    let conn = establish_connection();

    for _ in 0..5 {
        generate_post(&conn, "moto");
        generate_post(&conn, "roggen");
        generate_post(&conn, "tech");
    }
}

fn generate_post(conn: &PgConnection, blog: &str) {
    let mut rng = rand::thread_rng();
    let post = create_post(conn, blog, &format!("{} {}", blog, rng.gen::<u32>()));
    generate_headings(conn, post.id.clone());
    generate_texts(conn, post.id.clone());
}

fn generate_headings(conn: &PgConnection, post_id: i32) {
    for index in 0..5 {
        create_heading(
            conn,
            post_id.clone(),
            index * 2,
            &format!("Heading {}/{}", post_id, index),
            2,
        );
    }
}

fn generate_texts(conn: &PgConnection, post_id: i32) {
    for index in 0..5 {
        create_text(
            conn,
            post_id.clone(),
            index * 2 + 1,
            &format!("Text {}/{}", post_id, index),
        );
    }
}
