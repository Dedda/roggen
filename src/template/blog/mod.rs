use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use maud::Markup;
use rocket::Rocket;
use rocket::http::Cookies;

use crate::data::models::Post;
use crate::template::elements::Link;
use crate::template::page;
use crate::data::read::{load_posts, load_post};
use crate::template::blog::post::render_post;
use crate::get_lang;

pub mod post_renderable;
pub mod post;

lazy_static! {
    static ref BLOGS: Arc<Mutex<HashMap<String, Blog>>> = {
        let mut map = HashMap::new();
        map.insert("moto".to_string(), Blog::for_name("Moto"));
        map.insert("roggen".to_string(), Blog::for_name("Roggen"));
        map.insert("tech".to_string(), Blog::for_name("Tech"));
        Arc::new(Mutex::new(map))
    };
}

#[derive(Clone)]
struct Blog {
    name: String,
    path: String,
    title: Link,
}

impl Blog {
    fn for_name<S>(name: S) -> Blog where S: ToString {
        Blog {
            name: name.to_string(),
            path: name.to_string().to_ascii_lowercase(),
            title: Link::for_blog_name(name),
        }
    }
}

pub fn mount_blogs(rocket: Rocket) -> Rocket {
    rocket.mount("/blog", routes![post, blog_main, blog_list])
}

fn get_blog<S>(name: S) -> Option<Blog> where S: ToString {
    match BLOGS.lock().unwrap().get(&name.to_string()) {
        Some(b) => Some(b.clone()),
        None => None,
    }
}

fn embed_blog_contents(language: String, title: &Link, contents: Markup) -> Markup {
    page(language, title, contents)
}

fn blog_home(language: String, blog: Blog) -> Markup {
    let posts = load_posts(&blog.path);
    embed_blog_contents(language, &blog.title, posts_overview(&posts))
}

fn posts_overview(posts: &Vec<Post>) -> Markup {
    let items: Vec<Markup> = posts.iter().map(|p| post_overview_item(p)).collect();
    match posts.len() {
        0 => html! {
            p { "No posts in this blog yet :/" }
        },
        _ => html! {
            ul class="post-list" {
                @for item in items {
                    (item)
                }
            }
        },
    }
}

fn post_overview_item(post: &Post) -> Markup {
    let link = format!("/blog/{}/{}", post.blog, post.id);
    html! {
        li {
            div class="post-list-item" {
                a href=(link) { (post.title) }
            }
        }
    }
}

fn blog_not_found(name: &str) -> Markup {
    html! {
        div { p { (format!("Blog not found: {}", name)) } }
    }
}

#[get("/")]
fn blog_list(cookie: Cookies) -> Markup {
    page(get_lang(cookie), &Link::new("Blogs", "/blog"),
         html! {
        ul class="blog-list" {
            li { a href="/blog/moto" { "Moto" } }
            li { a href="/blog/roggen" { "Roggen" } }
            li { a href="/blog/tech" { "Tech" } }
            li { a href="/blog/punchlines" { "Punchlines" } }
        }
    })
}

#[get("/<name>")]
fn blog_main(name: String, cookie: Cookies) -> Markup {
    match get_blog(&name) {
        Some(b) => blog_home(get_lang(cookie), b),
        None => blog_not_found(&name),
    }
}

#[get("/<blog>/<id>")]
fn post(blog: String, id: i32, cookie: Cookies) -> Markup {
    let lang = get_lang(cookie);
    match get_blog(&blog) {
        Some(b) => {
            let contents = match load_post(id) {
                Some(post) => render_post(lang.clone(), &post),
                None => html! {
                    h1 { "Post not found!" }
                }
            };
            embed_blog_contents(lang, &b.title, contents)
        }
        None => blog_not_found(&blog),
    }
}