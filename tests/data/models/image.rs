use roggen::data::delete::{delete_image, delete_post};
use roggen::data::models::{NewImage, NewPost};
use roggen::data::read::load_image;
use roggen::data::write::{create_image, create_post};

#[test]
fn create_read_delete_image() {
    let new_post = NewPost::new("".to_string(), "".to_string());
    let post = create_post(&new_post);
    let section_index = 2;
    let image_name = "".to_string();
    let caption = "".to_string();
    let file_name = "".to_string();
    let img = NewImage {
        post: post.id.clone(),
        section_index: section_index.clone(),
        image_name: image_name.clone(),
        caption: caption.clone(),
        file_name: file_name.clone(),
    };
    let img = create_image(&img);
    assert_eq!(&post.id, &img.post);
    assert_eq!(&section_index, &img.section_index);
    assert_eq!(&image_name, &img.image_name);
    assert_eq!(&caption, &img.caption);
    assert_eq!(&file_name, &img.file_name);
    let loaded = load_image(img.id);
    assert_eq!(Some(img.clone()), loaded);
    delete_image(&img);
    let loaded = load_image(img.id);
    assert_eq!(None, loaded);
    delete_post(&post);
}
