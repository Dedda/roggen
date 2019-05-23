use std::ops::Deref;

use crate::data::models::*;
use crate::schema::*;
use crate::data::connections::pooled_connection;
use diesel::query_dsl::RunQueryDsl;

pub fn create_post(new_post: &NewPost) -> Post {
    diesel::insert_into(post::table)
        .values(new_post)
        .get_result(pooled_connection().deref())
        .expect("Error saving post")
}

pub fn create_image(new_image: &NewImage) -> Image {
    diesel::insert_into(image::table)
        .values(new_image)
        .get_result(pooled_connection().deref())
        .expect("Error saving image")
}

pub fn create_text_section(new_text_section: &NewTextSection) -> TextSection {
    diesel::insert_into(text_section::table)
        .values(new_text_section)
        .get_result(pooled_connection().deref())
        .expect("Error saving text section")
}

pub fn create_heading(new_heading: &NewHeading) -> Heading {
    diesel::insert_into(heading::table)
        .values(new_heading)
        .get_result(pooled_connection().deref())
        .expect("Error saving heading")
}