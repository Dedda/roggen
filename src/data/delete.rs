use crate::data::connections::pooled_connection;
use crate::data::models::*;
use crate::*;

use std::ops::Deref;

pub fn delete_post(old_post: &Post) {
    use crate::schema::post::dsl::*;

    diesel::delete(post.filter(id.eq(old_post.id)))
        .execute(pooled_connection().deref())
        .expect("Error deleting post");
}

pub fn delete_image(old_img: &Image) {
    use crate::schema::image::dsl::*;

    diesel::delete(image.filter(id.eq(old_img.id)))
        .execute(pooled_connection().deref())
        .expect("Error deleting image");
}

pub fn delete_text_section(old_text_section: &TextSection) {
    use crate::schema::text_section::dsl::*;

    diesel::delete(text_section.filter(id.eq(old_text_section.id)))
        .execute(pooled_connection().deref())
        .expect("Error deleting image");
}

pub fn delete_heading(old_heading: &Heading) {
    use crate::schema::heading::dsl::*;

    diesel::delete(heading.filter(id.eq(old_heading.id)))
        .execute(pooled_connection().deref())
        .expect("Error deleting image");
}
