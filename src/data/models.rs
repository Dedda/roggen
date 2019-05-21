pub trait BelongsToPost {
    fn post_id(&self) -> i32;
    fn index(&self) -> i32;
}

pub struct Image {
    pub id: Option<i32>,
    pub post: i32,
    pub index: i32,
    pub name: String,
    pub caption: String,
    pub file_name: String,
}

pub struct TextSection {
    pub id: Option<i32>,
    pub post: i32,
    pub index: i32,
    pub text: String,
}

pub struct Heading {
    pub id: Option<i32>,
    pub post: i32,
    pub index: i32,
    pub text: String,
    pub size: i8,
}

pub enum PostPart {
    Image(Image),
    Text(TextSection),
    Heading(Heading),
}

pub struct Post {
    pub id: Option<i32>,
    pub title: String,
}

impl BelongsToPost for Image {
    fn post_id(&self) -> i32 {
        self.post
    }

    fn index(&self) -> i32 {
        self.index
    }
}

impl BelongsToPost for TextSection {
    fn post_id(&self) -> i32 {
        self.post
    }

    fn index(&self) -> i32 {
        self.index
    }
}

impl BelongsToPost for Heading {
    fn post_id(&self) -> i32 {
        self.post
    }

    fn index(&self) -> i32 {
        self.index
    }
}