use std::ops::Deref;

use crate::data::models::{Post, Heading, Image, TextSection};
use crate::data::connections::pooled_connection;
use crate::diesel::prelude::*;

pub fn load_post(post_id: i32) -> Option<Post> {
    use crate::schema::post::dsl::*;

    let result = post
        .find(post_id)
        .load::<Post>(pooled_connection().deref())
        .expect("Error loading post");
    match result.first() {
        Some(p) => Some(p.clone()),
        None => None,
    }
}

pub fn load_posts(blog_id: &str) -> Vec<Post> {
    use crate::schema::post::dsl::*;

    post.filter(blog.eq(blog_id))
        .load::<Post>(pooled_connection().deref())
        .expect("Error loading posts")
}

pub fn load_headings(post_id: i32) -> Vec<Heading> {
    use crate::schema::heading::dsl::*;

    heading.filter(post.eq(post_id))
        .load::<Heading>(pooled_connection().deref())
        .expect("Error loading headings")
}

pub fn load_images(post_id: i32) -> Vec<Image> {
    use crate::schema::image::dsl::*;

    image.filter(post.eq(post_id))
        .load::<Image>(pooled_connection().deref())
        .expect("Error loading headings")
}

pub fn load_texts(post_id: i32) -> Vec<TextSection> {
    use crate::schema::text_section::dsl::*;

    text_section.filter(post.eq(post_id))
        .load::<TextSection>(pooled_connection().deref())
        .expect("Error loading headings")
}