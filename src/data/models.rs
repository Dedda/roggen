use crate::schema::*;

pub trait BelongsToPost {
    fn post_id(&self) -> i32;
    fn index(&self) -> i32;
}

#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub post: i32,
    pub section_index: i32,
    pub image_name: String,
    pub caption: String,
    pub file_name: String,
}

#[derive(Insertable)]
#[table_name="image"]
pub struct NewImage {
    pub post: i32,
    pub section_index: i32,
    pub image_name: String,
    pub caption: String,
    pub file_name: String,
}

#[derive(Queryable)]
pub struct TextSection {
    pub id: i32,
    pub post: i32,
    pub section_index: i32,
    pub section_text: String,
}

#[derive(Insertable)]
#[table_name="text_section"]
pub struct NewTextSection {
    pub post: i32,
    pub section_index: i32,
    pub section_text: String,
}

#[derive(Queryable)]
pub struct Heading {
    pub id: i32,
    pub post: i32,
    pub section_index: i32,
    pub heading_text: String,
    pub heading_size: i32,
}

#[derive(Insertable)]
#[table_name="heading"]
pub struct NewHeading {
    pub post: i32,
    pub section_index: i32,
    pub heading_text: String,
    pub heading_size: i32,
}

pub enum PostPart {
    Image(Image),
    Text(TextSection),
    Heading(Heading),
}

#[derive(Clone, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub published: bool,
    pub blog: String,
}

#[derive(Insertable)]
#[table_name="post"]
pub struct NewPost {
    pub title: String,
    pub published: bool,
    pub blog: String,
}

impl BelongsToPost for Image {
    fn post_id(&self) -> i32 {
        self.post
    }

    fn index(&self) -> i32 {
        self.section_index
    }
}

impl BelongsToPost for TextSection {
    fn post_id(&self) -> i32 {
        self.post
    }

    fn index(&self) -> i32 {
        self.section_index
    }
}

impl BelongsToPost for Heading {
    fn post_id(&self) -> i32 {
        self.post
    }

    fn index(&self) -> i32 {
        self.section_index
    }
}


impl BelongsToPost for NewImage {
    fn post_id(&self) -> i32 {
        self.post
    }

    fn index(&self) -> i32 {
        self.section_index
    }
}

impl BelongsToPost for NewTextSection {
    fn post_id(&self) -> i32 {
        self.post
    }

    fn index(&self) -> i32 {
        self.section_index
    }
}

impl BelongsToPost for NewHeading {
    fn post_id(&self) -> i32 {
        self.post
    }

    fn index(&self) -> i32 {
        self.section_index
    }
}