use std::ops::Deref;

use crate::data::models::Post;
use crate::data::connections::get_pool;
use crate::diesel::prelude::*;

pub fn load_post(post_id: i32) -> Option<Post> {
    use crate::schema::post::dsl::*;

    let pooled = get_pool().get().expect("Got no connection from pool");
    let connection = pooled.deref();
//    let result = post.find(post_id).load::<Post>().expect("Error loading post");
//    Some(result)
    let results: Vec<Post> = post.filter(id.eq(post_id))
        .limit(1)
        .load::<Post>(connection)
        .expect("Error loading posts");
    match results.first() {
        Some(p) => Some(p.clone()),
        None => None,
    }
}

pub fn load_posts(blog_id: &str) -> Vec<Post> {
    use crate::schema::post::dsl::*;

    let pooled = get_pool().get().expect("Got no connection from pool");
    let connection = pooled.deref();
    post.filter(blog.eq(blog_id))
        .load::<Post>(connection)
        .expect("Error loading posts")
}