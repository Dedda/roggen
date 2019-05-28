use roggen::data::write::{create_post, create_text_section};
use roggen::data::models::{NewPost, NewTextSection};
use roggen::data::delete::{delete_post, delete_text_section};
use roggen::data::read::load_text;

#[test]
fn create_read_delete_image() {
    let new_post = NewPost::new("".to_string(), "".to_string());
    let post = create_post(&new_post);
    let section_index = 2;
    let section_text = "".to_string();
    let text = NewTextSection {
        post: post.id.clone(),
        section_index: section_index.clone(),
        section_text: section_text.clone(),
    };
    let text = create_text_section(&text);
    assert_eq!(&post.id, &text.post);
    assert_eq!(&section_index, &text.section_index);
    assert_eq!(&section_text, &text.section_text);
    let loaded = load_text(text.id);
    assert_eq!(Some(text.clone()), loaded);
    delete_text_section(&text);
    let loaded = load_text(text.id);
    assert_eq!(None, loaded);
    delete_post(&post);
}