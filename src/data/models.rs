pub trait BelongsToPost {
    fn post_id(&self) -> i32;
    fn index(&self) -> i32;
}

pub struct Image {
    id: Option<i32>,
    post: i32,
    index: i32,
    name: String,
    caption: String,
    file_name: String,
}

pub struct TextSection {
    id: Option<i32>,
    post: i32,
    index: i32,
    text: String,
}

pub struct Heading {
    id: Option<i32>,
    post: i32,
    index: i32,
    text: String,
    size: i8,
}

pub enum PostPart {
    Image(Image),
    Text(TextSection),
    Heading(Heading),
}

pub struct Post {
    id: Option<i32>

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