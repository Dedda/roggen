use maud::Markup;
use rocket::Rocket;

use crate::data::models::Post;
use crate::template::elements::Link;
use crate::template::page;

pub mod post_renderable;
pub mod moto;
pub mod post;
pub mod roggen;
pub mod tech;

pub fn embed_blog_contents(title: &Link, contents: Markup) -> Markup {
    page(title, contents)
}

pub fn blog_home(title: &Link, posts: &Vec<Post>) -> Markup {
    embed_blog_contents(title, html! {
        (posts_overview(posts))
    })
}

pub fn mount_blogs(rocket: Rocket) -> Rocket {
    let rocket = moto::mount(rocket);
    let rocket = roggen::mount(rocket);
    let rocket = tech::mount(rocket);
    rocket.mount("/blog", routes![blog_list])
}

pub fn posts_overview(posts: &Vec<Post>) -> Markup {
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

#[get("/")]
fn blog_list() -> Markup {
    page(&Link::new("Blogs", "/blog"),
        html! {
        ul class="blog-list" {
            li { a href="/blog/moto" { "Moto" } }
            li { a href="/blog/roggen" { "Roggen" } }
            li { a href="/blog/tech" { "Tech" } }
        }
    })
}