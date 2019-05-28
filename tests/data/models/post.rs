use roggen::data::write::create_post;
use roggen::data::models::NewPost;
use roggen::data::delete::delete_post;
use roggen::data::read::load_post;

#[test]
fn create_read_delete_post() {
    let title = "Test Post".to_string();
    let blog = "testblog".to_string();
    let new_post = NewPost::new(title.clone(), blog.clone());
    let post = create_post(&new_post);
    assert_eq!(&title, &post.title);
    assert_eq!(&blog, &post.blog);
    let id = post.id.clone();
    let loaded = load_post(id);
    assert_eq!(Some(post.clone()), loaded);
    delete_post(&post);
    let loaded = load_post(id);
    assert_eq!(None, loaded);
}