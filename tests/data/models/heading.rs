use roggen::data::delete::{delete_heading, delete_post};
use roggen::data::models::{NewHeading, NewPost};
use roggen::data::read::load_heading;
use roggen::data::write::{create_heading, create_post};

#[test]
fn create_read_delete_heading() {
    let new_post = NewPost::new("".to_string(), "".to_string());
    let post = create_post(&new_post);
    let section_index = 2;
    let heading_text = "".to_string();
    let heading_size = 5;
    let heading = NewHeading {
        post: post.id.clone(),
        section_index: section_index.clone(),
        heading_text: heading_text.clone(),
        heading_size: heading_size.clone(),
    };
    let heading = create_heading(&heading);
    assert_eq!(&post.id, &heading.post);
    assert_eq!(&section_index, &heading.section_index);
    assert_eq!(&heading_text, &heading.heading_text);
    assert_eq!(&heading_size, &heading.heading_size);
    let loaded = load_heading(heading.id);
    assert_eq!(Some(heading.clone()), loaded);
    delete_heading(&heading);
    let loaded = load_heading(heading.id);
    assert_eq!(None, loaded);
    delete_post(&post);
}
