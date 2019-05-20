pub struct Image {
    id: Option<i32>,

}

pub struct TextSection {
    id: Option<i32>
}

pub enum PostPart {
    Image(Image),
    Text(TextSection),
}

pub struct Post {
    id: Option<i32>

}