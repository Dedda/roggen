use maud::Markup;
use rocket::Rocket;

use crate::data::models::Post;
use crate::template::elements::Link;
use crate::template::page;

pub mod moto;
pub mod roggen;
pub mod tech;

pub fn embed_blog_contents(title: &Link, contents: Markup) -> Markup {
    page(title, contents)
}

pub fn blog_home(title: &Link, posts: Vec<Post>) -> Markup {
    embed_blog_contents(title, html! {
        ul {
            @for post in &posts {
                li { (post.title) }
            }
        }
    })
}

pub fn mount_blogs(rocket: Rocket) -> Rocket {
    let rocket = moto::mount(rocket);
    let rocket = roggen::mount(rocket);
    let rocket = tech::mount(rocket);
    rocket.mount("/blog", routes![blog_list])
}

#[get("/")]
fn blog_list() -> Markup {
    page(&Link::new("Blogs", "/blog"),
        html! {
        ul {
            li { a href="/blog/moto" { "Moto" } }
            li { a href="/blog/roggen" { "Roggen" } }
            li { a href="/blog/tech" { "Tech" } }
        }
    })
}